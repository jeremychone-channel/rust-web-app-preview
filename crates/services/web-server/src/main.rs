// #![allow(unused)] // For early development stages.

// region:    --- Modules

mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
pub use lib_core::config;

use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_req_stamp::mw_req_stamp;
use crate::web::mw_res_map::mw_response_map;
use crate::web::{routes_login, routes_static, rpc};
use axum::{middleware, Router};
use lib_core::_dev_utils;
use lib_core::model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	_dev_utils::init_dev().await;

	// -- Initialize ModelController
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let routes_rpc =
		rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_response_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(middleware::from_fn(mw_req_stamp))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	info!("{:<12} - on {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
