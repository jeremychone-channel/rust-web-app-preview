use crate::crypt::token::{validate_token_sign_and_exp, Token};
use crate::ctx::Ctx;
use crate::model::user::UserBmc;
use crate::model::ModelManager;
use crate::web::AUTH_TOKEN;
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

	let token = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string());

	// region:    --- Parse & Validate Token to Ctx
	let token = token
		.ok_or(CtxAuthError::TokenNotInCookie)
		.and_then(|t| Token::parse(&t).map_err(|_| CtxAuthError::TokenWrongFormat));

	let result_user = match &token {
		Ok(token) => {
			UserBmc::get_for_auth_by_username(&mm, &Ctx::root_ctx(), &token.user)
				.await?
				.ok_or(CtxAuthError::FailFoundUser(token.user.to_string()))
		}
		Err(ex) => CtxAuthResult::Err(ex.clone()),
	};

	let result_ctx = result_user
		.and_then(|user| {
			validate_token_sign_and_exp(
				&token.unwrap(),
				&user.token_salt.to_string(),
			)
			.map(|_| user)
			.map_err(|ex| CtxAuthError::FailValidate(ex.to_string()))
		})
		.and_then(|user| {
			Ctx::new(user.id)
				.map_err(|ex| CtxAuthError::CtxCreateFail(ex.to_string()))
		});
	// endregion: --- Parse & Validate Token to Ctx

	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(CtxAuthError::TokenNotInCookie))
	{
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		println!("->> {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxAuthResult<Ctx>>()
			.ok_or(Error::CtxAuth(CtxAuthError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxAuth)
	}
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxAuthResult<T> = core::result::Result<T, CtxAuthError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxAuthError {
	TokenNotInCookie,
	TokenWrongFormat,
	FailValidate(String),
	FailFoundUser(String),
	CtxNotInRequestExt,
	CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
