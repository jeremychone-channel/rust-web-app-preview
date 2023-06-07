use crate::crypt::token::{validate_token_sign_and_exp, Token};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForAuth};
use crate::model::ModelManager;
use crate::web;
use crate::web::AUTH_TOKEN;
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use std::str::FromStr;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_require<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	// -- Parse Token
	let token = token.ok_or(CtxAuthError::TokenNotInCookie).and_then(|t| {
		Token::from_str(&t).map_err(|_| CtxAuthError::TokenWrongFormat)
	});

	// -- Validate Token
	// Get the user from the db.
	let result_user = match &token {
		Ok(token) => {
			UserBmc::first_by_username::<UserForAuth>(
				&Ctx::root_ctx(),
				&mm,
				&token.user,
			)
			.await? // If cannot access the DB, critical enough to return Error. TODO: To reassess.
			.ok_or(CtxAuthError::FailUserNotFound(token.user.to_string()))
		}
		Err(ex) => CtxAuthResult::Err(ex.clone()),
	};

	let result_user = result_user.and_then(|user| {
		validate_token_sign_and_exp(&token.unwrap(), &user.token_salt.to_string())
			.map(|_| user)
			.map_err(|ex| CtxAuthError::FailValidate(ex.to_string()))
	});

	// -- Update Token
	// If auth success, create a new Token with the updated expiration date.
	if let Ok(user) = result_user.as_ref() {
		web::set_token_cookie(
			&cookies,
			&user.username,
			&user.token_salt.to_string(),
		)?;
	}
	// Ohterwise, remove the cookie if something went wrong other than TokenNotInCookie.
	else if !matches!(result_user, Err(CtxAuthError::TokenNotInCookie)) {
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// -- Create the ctx if we have the user.
	let result_ctx = result_user.and_then(|user| {
		Ctx::new(user.id).map_err(|ex| CtxAuthError::CtxCreateFail(ex.to_string()))
	});

	// -- Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!("{:<12} - Ctx", "EXTRACTOR");

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
	FailUserNotFound(String),
	FailValidate(String),
	CtxNotInRequestExt,
	CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
