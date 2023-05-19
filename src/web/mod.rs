// region:    --- Modules

mod error;
pub mod mw_auth;
pub mod mw_req_stamp;
pub mod mw_res_mapper;
pub mod routes_login;
pub mod routes_static;
pub mod rpc;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

use crate::crypt::token::generate_token;
use time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
	let token = generate_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
	cookie.set_http_only(true);

	cookies.add(cookie);

	Ok(())
}

/// Request "Stamp" created at the beginning of the http request
/// flow by the `mw_req_stamp_resolver` with the following properties:
///
/// - uuid    - Unique identifier of the request.
/// - time_in - Equivalent (close enough) of the beginning of the request.
///
#[derive(Debug, Clone)]
pub struct ReqStamp {
	pub uuid: Uuid,
	pub time_in: OffsetDateTime,
}
