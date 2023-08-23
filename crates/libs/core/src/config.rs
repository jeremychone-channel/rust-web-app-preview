use lib_base::env::{self, get_env, get_env_b64u_as_u8s, get_env_parse};
use std::sync::OnceLock;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			// -- Crypt
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,

			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- Web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	ConfigEnvFail(env::Error),
}

// region:    --- Froms
// FIXME: This assume that all env error will be a configEnvFail error.
//        Would make sense if config::error only.
impl From<env::Error> for Error {
	fn from(val: env::Error) -> Self {
		Self::ConfigEnvFail(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error
