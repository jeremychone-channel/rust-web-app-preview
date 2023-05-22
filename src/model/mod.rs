//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type structure and access.
//! - All application code data access must go through the Model layer to access any data.
//! - `ModelManager` holds the internal states/resources for data access.
//!   (e.g., db_pool, S3 client, redis client).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - It's designed to be passed as an argument to all Backend Model Controllers (aka, `Bmc`)
//!   (e.g., `TaskBmc`)

// region:    --- Modules

mod base;
pub mod error;
mod store;
pub mod task;
pub mod user;

pub use self::error::{Error, Result};

use crate::model::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	/// Constructor
	pub async fn new() -> Result<Self> {
		let db: Db = new_db_pool().await?;

		Ok(Self { db })
	}

	///
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
