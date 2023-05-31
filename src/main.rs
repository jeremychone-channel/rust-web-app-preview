// #![allow(unused)] // For early development stages.

// region:    --- Modules

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use crate::web::mw_req_stamp::mw_req_stamp_resolver;
use axum::{middleware, Router};
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
	let routes_rpc = web::rpc::routes(mm.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_ctx_require));

	let routes_all = Router::new()
		.merge(web::routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(
			web::mw_res_mapper::mw_response_mapper,
		))
		.layer(middleware::from_fn_with_state(
			mm.clone(),
			web::mw_auth::mw_ctx_resolver,
		))
		.layer(middleware::from_fn(mw_req_stamp_resolver))
		.layer(CookieManagerLayer::new())
		.fallback_service(web::routes_static::serve_dir());

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
