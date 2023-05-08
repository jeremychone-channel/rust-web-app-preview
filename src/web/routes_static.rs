use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::{any_service, get_service, MethodRouter};
use axum::Router;
use tower_http::services::ServeDir;

// See: https://github.com/tokio-rs/axum/issues/1931#issuecomment-1506067949
pub fn serve_dir() -> MethodRouter {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Not found")
	}

	any_service(
		ServeDir::new("web-folder/").not_found_service(handle_404.into_service()),
	)
}
