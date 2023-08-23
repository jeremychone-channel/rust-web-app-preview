use crate::web;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lib_base::token;
use lib_core::{model, pwd};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- RPC
	RpcMethodUnknown(String),
	RpcMissingParams { rpc_method: String },
	RpcFailJsonParams { rpc_method: String },

	// -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFailPwdNotMatching { user_id: i64 },

	// -- Middelware/Extractor
	ReqStampNotInResponseExt,

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),

	// -- Modules
	Model(model::Error),
	Crypt(pwd::Error),
	Token(token::Error),

	// -- External Modules
	SerdeJson(String),
}

// region:    --- Error Froms
impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Error::Model(val)
	}
}

impl From<pwd::Error> for Error {
	fn from(val: pwd::Error) -> Self {
		Self::Crypt(val)
	}
}

impl From<token::Error> for Error {
	fn from(val: token::Error) -> Self {
		Self::Token(val)
	}
}

impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Error::SerdeJson(val.to_string())
	}
}

// endregion: --- Error Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl Error {
	/// Error to ClientError and HTTP Status code
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::Error::*;

		match self {
			// -- Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Model
			Model(model::Error::EntityNotFound { entity, id }) => (
				StatusCode::BAD_REQUEST,
				ClientError::ENTITY_NOT_FOUND { entity, id: *id },
			),
			Model(model::Error::UserAlreadyExists { .. }) => {
				(StatusCode::BAD_REQUEST, ClientError::USER_ALREADY_EXISTS)
			}

			// -- Fallback
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

/// This is the ClientError used to be serialized in the
/// json-rpc error body.
/// Only used in the mw_res_mapper
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
	USER_ALREADY_EXISTS,
	SERVICE_ERROR,
}
