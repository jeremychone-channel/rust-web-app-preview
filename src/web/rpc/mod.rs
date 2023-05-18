// region:    --- Modules

mod task_rpc;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::rpc::task_rpc::{create_task, delete_task, list_tasks, update_task};
use crate::web::{Error, Result};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

// endregion: --- Modules

/// RPC Request Body coming from the client RPC Call.
#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

/// RPC Context holding the id and method for further logging.
#[derive(Debug)]
pub struct RpcCtx {
	pub id: Option<Value>,
	pub method: String,
}

// region:    --- Params Types
#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}
// endregion: --- Params Types

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

macro_rules! exec_rpc_fn {
	// With params.
	($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
		let rpc_fn_name = stringify!($rpc_fn);
		let params = $rpc_params.ok_or(Error::RpcMissingParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;
		let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	// Without params.
	($rpc_fn:expr, $ctx:expr, $mm:expr) => {{
		$rpc_fn($ctx, $mm).await.map(to_value)??
	}};
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: Ctx,
	Json(rpc_req): Json<RpcRequest>,
) -> Response {
	// -- Create the RPC Context to be set to the response.extensions.
	let rpc_ctx = RpcCtx {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	// -- Execute the RPC Handler routing.
	let mut res = rpc_handler_inner(ctx, mm, rpc_req).await.into_response();

	// -- Set the RPC Context as a reponse extension.
	res.extensions_mut().insert(rpc_ctx);

	res
}

async fn rpc_handler_inner(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
) -> Result<Json<Value>> {
	let RpcRequest {
		id: rpc_id,
		method: rpc_method,
		params: rpc_params,
	} = rpc_req;

	debug!(
		"{:<12} - rpc_handler_inner - method: {rpc_method}",
		"HANDLER"
	);

	let result_body = match rpc_method.as_str() {
		// -- Task RPC Methods
		"create_task" => exec_rpc_fn!(create_task, ctx, mm, rpc_params),
		"list_tasks" => exec_rpc_fn!(list_tasks, mm, ctx),
		"update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
		"delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),

		// -- Fallback
		_ => return Err(Error::RpcMethodUnkown(rpc_method)),
	};

	let body_response = json!({
		"id": rpc_id,
		"result": result_body
	});

	Ok(Json(body_response))
}
