// region:    --- Modules

mod error;
pub mod mw_auth;
pub mod mw_req_stamp;
pub mod mw_res_map;
pub mod routes_login;
pub mod routes_static;
pub mod rpc;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

use lib_core::crypt::token::generate_web_token;
use time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
	let token = generate_web_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
	cookie.set_http_only(true);
	cookie.set_path("/");

	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
	let mut cookie = Cookie::named(AUTH_TOKEN);
	cookie.set_path("/");

	cookies.remove(cookie);

	Ok(())
}

/// Resolve by mw_req_stamp.
#[derive(Debug, Clone)]
pub struct ReqStamp {
	pub uuid: Uuid,
	pub time_in: OffsetDateTime,
}
