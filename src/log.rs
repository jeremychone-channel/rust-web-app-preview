use crate::ctx::Ctx;
use crate::error::ClientError;
use crate::utils::now_utc_str;
use crate::web::rpc::RpcCtx;
use crate::{Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub async fn log_request(
	uuid: Uuid,
	http_method: Method,
	uri: Uri,
	rpc_ctx: Option<&RpcCtx>,
	ctx: Option<Ctx>,
	service_error: Option<&Error>,
	client_error: Option<ClientError>,
) -> Result<()> {
	let error_type = service_error.map(|se| se.as_ref().to_string());
	let error_data = serde_json::to_value(service_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take()));

	// Create the RequestLogLine
	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: now_utc_str(),

		http_path: uri.to_string(),
		http_method: http_method.to_string(),

		rpc_id: rpc_ctx.and_then(|rpc| rpc.id.as_ref().map(|id| id.to_string())),
		rpc_method: rpc_ctx.map(|rpc| rpc.method.to_string()),

		user_id: ctx.map(|c| c.user_id()),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),

		error_type,
		error_data,
	};

	println!("   ->> log_request: \n{}", json!(log_line));

	// TODO - Send to cloud-watch.

	Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
	uuid: String,      // uuid string formatted
	timestamp: String, // (should be iso8601)

	// -- User and context attributes.
	user_id: Option<i64>,

	// -- http request attributes.
	http_path: String,
	http_method: String,

	// -- rpc attributes.
	rpc_id: Option<String>,
	rpc_method: Option<String>,

	// -- Errors attributes.
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}
