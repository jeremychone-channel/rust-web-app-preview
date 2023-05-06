use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web::rpc::RpcCtx;
use crate::web::ReqStamp;
use crate::Error;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::{debug, error};

pub async fn main_response_mapper(
	uri: Uri,
	req_stamp: ReqStamp,
	http_method: Method,
	ctx: Option<Ctx>,
	res: Response,
) -> Response {
	debug!("{:<12} - main_response_mapper", "RES_MAPPER");

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": req_stamp.uuid.to_string(),
					}
				});

				debug!("client_error_body:\n{client_error_body}");

				// Build the new response from the client_error_body.
				(*status_code, Json(client_error_body)).into_response()
			});

	// -- Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	let rpc_ctx = res.extensions().get::<RpcCtx>();
	if let Err(log_err) = log_request(
		req_stamp,
		http_method,
		uri,
		rpc_ctx,
		ctx,
		service_error,
		client_error,
	)
	.await
	{
		error!("CRITICAL ERROR - COULD NOT LOG_REQUEST - {log_err:?}");
	};

	// The empty line.
	debug!("");

	// -- Return the appropriate response.
	error_response.unwrap_or(res)
}
