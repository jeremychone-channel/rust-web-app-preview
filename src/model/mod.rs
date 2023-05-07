//! Simplistic Model Layer
//! (with mock-store layer)

// region:    --- Modules
// -- Sub-Modules
mod base;
pub mod error;
mod store;
pub mod ticket;
pub mod user;

// -- Re-Exports
pub use self::error::{Error, Result};
pub use self::store::init_dev_db::init_dev_db;

// -- Imports
use crate::conf;
use crate::model::store::Db;
use sqlx::postgres::PgPoolOptions;
// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	/// Constructor
	/// FIXME - Use once_cell or something To make sure it is call only once.
	///         But should probably failed is call twice.
	pub async fn new() -> Result<Self> {
		let db: Db = PgPoolOptions::new()
			.max_connections(5)
			.connect(&conf().DB_URL)
			.await
			.map_err(|ex| Error::DbFailToCreatePool(ex.to_string()))?;

		Ok(Self { db })
	}

	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
