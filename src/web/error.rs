use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	// -- RPC
	RpcMethodUnkown(String),
	RpcMissingParams { rpc_method: String },
	RpcFailJsonParams { rpc_method: String },

	// -- Middelware/Extractor
	ReqStampNotInResponseExt,

	// -- Login
	LoginFail,
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { username: String },

	// -- Auth
	AuthFailUserNotFound,
	CtxAuth(crate::web::mw_auth::CtxAuthError),

	// -- Sub Modules
	Crypt(crate::crypt::Error),
	Model(crate::model::Error),

	// -- External Modules
	SerdeJson(String),
}

// region:    --- Error Froms
impl From<crate::model::Error> for Error {
	fn from(val: crate::model::Error) -> Self {
		Error::Model(val)
	}
}

impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Error::SerdeJson(val.to_string())
	}
}

impl From<crate::crypt::Error> for Error {
	fn from(val: crate::crypt::Error) -> Self {
		Error::Crypt(val)
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
		response
			.extensions_mut()
			.insert(crate::Error::Web(self));

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
