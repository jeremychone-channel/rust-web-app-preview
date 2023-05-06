use crate::crypt::pwd::{self};
use crate::crypt::token::generate_token;
use crate::crypt::EncryptArgs;
use crate::ctx::Ctx;
use crate::model::user::UserBmc;
use crate::model::ModelManager;
use crate::web;
use crate::web::{Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.with_state(mm)
}

// region:    --- Handler
async fn api_login_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_login", "HANDLER");

	let LoginPayload { username, pwd: pwd_clear } = payload;

	// -- Get the user.
	let user = UserBmc::get_for_auth_by_username(&mm, &Ctx::root_ctx(), &username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;

	// -- Validate the password.
	pwd::validate_pwd(
		&EncryptArgs {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear,
		},
		&user.pwd,
	)?;

	// -- Generate the web token.
	let token = generate_token(&user.username, &user.token_salt.to_string())?;
	cookies.add(Cookie::new(web::AUTH_TOKEN, token.to_string()));

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}
// endregion: --- Handler
