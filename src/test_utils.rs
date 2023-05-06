use crate::model::{init_dev_db, ModelManager};
use std::sync::Once;
use tokio::sync::OnceCell;
use tracing::info;
use tracing_subscriber::EnvFilter;

// Initializer for local development
// (for early development, called from `main()`)
pub async fn init_dev_all() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			info!("{:<12} - init_dev()", "FOR-DEV-ONLY");

			init_dev_db().await.unwrap();

			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

// For unit tests.
pub async fn init_test_all() -> ModelManager {
	let mm = init_dev_all().await;

	static INIT_TRACING: Once = Once::new();

	INIT_TRACING.call_once(init_test_tracing);

	mm
}

// In case unit tests does not need a DB but wants tracing.
pub fn init_test_tracing() {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();
}
