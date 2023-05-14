use crate::ctx::Ctx;
use crate::model::base::{create, delete, get, list, update, DbBmc};
use crate::model::{ModelManager, Result};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::Fields;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// region:    --- Task Types
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Task {
	pub id: i64,

	pub title: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}
// endregion: --- Task Types

pub struct TaskBmc;

impl DbBmc for TaskBmc {
	const TABLE: &'static str = "task";
	const HAS_TIMESTAMPS: bool = true;
}

impl TaskBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		task_fc: TaskForCreate,
	) -> Result<i64> {
		create::<Self, _>(ctx, mm, task_fc).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
		get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
		list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		task_fu: TaskForUpdate,
	) -> Result<()> {
		update::<Self, _>(ctx, mm, id, task_fu).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		delete::<Self>(ctx, mm, id).await
	}
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use crate::_dev_utils;
	use crate::model;
	use anyhow::Result;

	#[tokio::test]
	async fn test_create() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let root_ctx = Ctx::root_ctx();
		let title = "TEST TITLE - test_model_task_create";

		// -- Exec - Create
		let id = TaskBmc::create(
			&root_ctx,
			&mm,
			TaskForCreate { title: title.to_string() },
		)
		.await?;

		// -- Check - Create
		let task = TaskBmc::get(&root_ctx, &mm, id).await?;
		assert_eq!(title, task.title);

		// -- Clean - Delete
		TaskBmc::delete(&root_ctx, &mm, id).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_delete_err() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let root_ctx = Ctx::root_ctx();
		let id = 10; // below 1000 so should have no row.

		// -- Exec
		let res = TaskBmc::delete(&root_ctx, &mm, id).await;

		// -- Check
		assert!(
			matches!(res, Err(model::Error::EntityNotFound { table: "task", id })),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
