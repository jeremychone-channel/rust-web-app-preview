use crate::model::{init_dev_db, ModelManager};
use tokio::sync::OnceCell;

pub async fn init_dev_all() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			println!("{:<12} - init_dev()", "FOR-DEV-ONLY");

			init_dev_env();

			init_dev_db().await.unwrap();

			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub fn init_dev_env() {
	dotenvy::dotenv().unwrap();
}
