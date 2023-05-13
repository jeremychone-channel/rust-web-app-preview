use crate::{crypt, model, web};
use axum::http::StatusCode;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Conf
	ConfMissingEnv(&'static str),
	ConfWrongFormat(&'static str),
	FailToLoadConf(&'static str),

	// -- Sub-Modules
	Web(web::Error),
	Crypt(crypt::Error),
	Model(model::Error),

	// -- Utils
	FailToB64UDecode,
	DateFailParse(String),

	// -- Others
	CtxCannotNewRootCtx,
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter,
	) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Error Froms
impl From<crate::crypt::Error> for Error {
	fn from(val: crate::crypt::Error) -> Self {
		Error::Crypt(val)
	}
}

impl From<crate::model::Error> for Error {
	fn from(val: crate::model::Error) -> Self {
		Error::Model(val)
	}
}
// endregion: --- Error Froms

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			// -- Web
			Self::Web(web::Error::LoginFail) => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}
			Self::Web(web::Error::CtxAuth(_)) => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}
			Self::Web(web::Error::Model(model::Error::EntityNotFound {
				entity: typ,
				id,
			})) => (
				StatusCode::BAD_REQUEST,
				ClientError::EntityNotFound { entity: typ, id: *id },
			),
			Self::Web(web::Error::Model(model::Error::UserAlreadyExists {
				..
			})) => (StatusCode::BAD_REQUEST, ClientError::USER_ALREADY_EXISTS),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	USER_ALREADY_EXISTS,
	LOGIN_FAIL,
	NO_AUTH,
	EntityNotFound { entity: &'static str, id: i64 },
	INVALID_PARAMS,
	SERVICE_ERROR,
}
