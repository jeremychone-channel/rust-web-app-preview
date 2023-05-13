// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use crate::conf;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion: --- Modules

// region:    --- Db Store

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&conf().DB_URL)
		.await
		.map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}

// endregion: --- Db Store
