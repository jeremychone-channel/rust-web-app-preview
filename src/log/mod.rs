use crate::ctx::Ctx;
use crate::utils::{format_time, now_utc};
use crate::web::rpc::RpcInfo;
use crate::web::ClientError;
use crate::web::{self, ReqStamp};
use crate::Result;
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use time::Duration;
use tracing::debug;

pub async fn log_request(
	http_method: Method,
	uri: Uri,
	req_stamp: ReqStamp,
	rpc_info: Option<&RpcInfo>,
	ctx: Option<Ctx>,
	web_error: Option<&web::Error>,
	client_error: Option<ClientError>,
) -> Result<()> {
	let ReqStamp { uuid, time_in } = req_stamp;

	let error_type = web_error.map(|se| se.as_ref().to_string());
	let error_data = serde_json::to_value(web_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take()));

	let now = now_utc();
	let duration: Duration = now - time_in;
	// duration_ms in milliseconds with microseconds precision.
	let duration_ms = (duration.as_seconds_f64() * 100000.).floor() / 1000.;

	// Create the RequestLogLine
	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: format_time(now), // LogLine timestamp ("time_out")

		time_in: format_time(time_in),
		duration_ms,

		http_path: uri.to_string(),
		http_method: http_method.to_string(),

		rpc_id: rpc_info.and_then(|rpc| rpc.id.as_ref().map(|id| id.to_string())),
		rpc_method: rpc_info.map(|rpc| rpc.method.to_string()),

		user_id: ctx.map(|c| c.user_id()),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),

		error_type,
		error_data,
	};

	debug!("REQUEST LOG LINE:\n{}", json!(log_line));

	// TODO - Send to cloud-watch.

	Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
	uuid: String,      // uuid string formatted
	timestamp: String, // (ref3339)

	time_in: String,
	duration_ms: f64,

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
