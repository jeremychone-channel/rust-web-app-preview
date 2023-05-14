use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web::rpc::RpcCtx;
use crate::web::{self, ReqStamp};
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, to_value};
use tracing::{debug, error};

pub async fn main_response_mapper(
	uri: Uri,
	req_stamp: ReqStamp,
	http_method: Method,
	ctx: Option<Ctx>,
	res: Response,
) -> Response {
	debug!("{:<12} - main_response_mapper", "RES_MAPPER");

	let rpc_ctx = res.extensions().get::<RpcCtx>();

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<web::Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error = to_value(client_error).unwrap();
				let message = client_error.get("message");
				let detail = client_error.get("detail");

				let client_error_body = json!({
						"id": rpc_ctx.as_ref().map(|rpc| rpc.id.clone()),
						"error": {
							// TODO: Will need to follow json-rpc error code practices.
							"code": 0,
							// In our design error.message == enum variant name
							"message": message,
							"data": {
								"req_uuid": req_stamp.uuid.to_string(),
								"detail": detail
							}
						}

				});

				debug!("CLIENT ERROR BODY:\n{client_error_body}");

				// Build the new response from the client_error_body.
				(*status_code, Json(client_error_body)).into_response()
			});

	// -- Build and log the server log line.
	let client_error = client_status_error.unzip().1;
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
	debug!("\n");

	// -- Return the appropriate response.
	error_response.unwrap_or(res)
}
