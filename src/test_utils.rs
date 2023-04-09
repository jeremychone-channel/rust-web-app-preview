use crate::model::ModelManager;
use std::env;
use std::sync::Once;
use tokio::sync::OnceCell;

const DEV_ENV: &str = r#"
SERVICE_DB_URL=postgres://app_user:dev_only_pwd@localhost/app_db
SERVICE_KEY_PWD=U96vOyRaI4tjumjHRk0FK2D1N1UAg2jiVZ66y-3Q0k_BfgY3Gmvft0A2JDzb9ZgT2QzGPgBUJnGtc_1MBeUS5w
SERVICE_KEY_TOKEN=CUF2rzJgVUSMYKls9ysmUGbZlha7H-HvqjHroY_wYPuUZsXqz7wpkGn3XVubVY8wfhLH7H8_0ksxOMkJiSiCWQ
"#;

pub async fn init_dev_all() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			println!("{:<12} - init_dev()", "FOR-DEV-ONLY");

			init_dev_envs();

			crate::model::init_dev_db().await.unwrap();

			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub fn init_dev_envs() {
	println!("{:<12} - init_dev_envs()", "FOR-DEV-ONLY");

	static INIT: Once = Once::new();

	INIT.call_once(|| {
		DEV_ENV
			.split('\n')
			.filter_map(|s| s.trim().split_once('='))
			.for_each(|(name, val)| env::set_var(name, val));
	});
}
