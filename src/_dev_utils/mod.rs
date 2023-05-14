// region:    --- Modules

mod dev_db;

use crate::model::ModelManager;
use tokio::sync::OnceCell;
use tracing::info;

// endregion: --- Modules

// Initializer for local development
// (for early development, called from `main()`)
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();
	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}

pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}
