use crate::{Error, Result};
use once_cell::sync::OnceCell;
use std::env;

pub fn conf() -> &'static Conf {
	static INSTANCE: OnceCell<Conf> = OnceCell::new();

	INSTANCE.get_or_init(|| {
		Conf::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct Conf {
	// -- Crypt
	pub KEY_PWD: Vec<u8>,
	pub KEY_TOKEN: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Conf {
	fn load_from_env() -> Result<Conf> {
		Ok(Conf {
			// -- Crypt
			KEY_PWD: get_env_b64u_as_u8s("SERVICE_KEY_PWD")?,
			KEY_TOKEN: get_env_b64u_as_u8s("SERVICE_KEY_TOKEN")?,
			TOKEN_DURATION_SEC: 1800.,

			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- Web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::ConfMissingEnv(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfWrongFormat(name))
}
