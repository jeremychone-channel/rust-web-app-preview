//! Model Layer
//! Design:
//! - The Model layer normalizes the application's data type structure and access.
//! - All application code data access must go through the Model layer.
//! - In a web-app setting, `ModelManager` holds the internal model states.
//! - It's used to call the entity Backend Model Controllers (aka, `Bmc`), e.g., `TicketBmc`.

// region:    --- Modules

mod base;
pub mod error;
mod store;
pub mod task;
pub mod user;

pub use self::error::{Error, Result};

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
