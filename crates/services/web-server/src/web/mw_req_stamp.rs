use crate::web::{Error, ReqStamp, Result};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lib_utils::time::now_utc;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_req_stamp<B>(
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_req_stamp_resolver", "MIDDLEWARE");

	let time_in = now_utc();
	let uuid = Uuid::new_v4();

	req.extensions_mut().insert(ReqStamp { uuid, time_in });

	Ok(next.run(req).await)
}

// region:    --- ReqStamp Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ReqStamp {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!("{:<12} - ReqStamp", "EXTRACTOR");

		parts
			.extensions
			.get::<ReqStamp>()
			.cloned()
			.ok_or(Error::ReqStampNotInResponseExt)
	}
}
// endregion: --- ReqStamp Extractor
