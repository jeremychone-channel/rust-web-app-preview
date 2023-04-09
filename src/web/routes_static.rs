use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

pub fn routes() -> Router {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Not found")
	}

	Router::new().nest_service(
		"/",
		get_service(
			ServeDir::new("src").not_found_service(handle_404.into_service()),
		),
	)
}
