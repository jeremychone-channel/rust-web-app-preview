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
	use crate::model::Error;
	use anyhow::Result;

	#[tokio::test]
	async fn test_create_basic() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_model_task_create_basic";

		// -- Exec
		let task_fc = TaskForCreate { title: fx_title.to_string() };
		let id = TaskBmc::create(&ctx, &mm, task_fc).await?;

		// -- Check
		let task = TaskBmc::get(&ctx, &mm, id).await?;
		assert_eq!(task.title, fx_title);

		// -- Clean
		TaskBmc::delete(&ctx, &mm, id).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = TaskBmc::get(&ctx, &mm, 100).await;

		// -- Check
		assert!(
			matches!(res, Err(Error::EntityNotFound { table: "task", id: fx_id })),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_list_basic() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_titles = &["test_list_basic 01", "test_list_basic 02"];
		let fx_tasks = _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

		// -- List
		let tasks = TaskBmc::list(&ctx, &mm).await?;
		let tasks: Vec<Task> = tasks
			.into_iter()
			.filter(|t| t.title.starts_with("test_list_basic"))
			.collect();
		assert_eq!(tasks.len(), 2, "number of seeded tasks.");

		// -- Clean
		for task in tasks.iter() {
			TaskBmc::delete(&ctx, &mm, task.id).await?;
		}

		Ok(())
	}

	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = TaskBmc::delete(&ctx, &mm, 100).await;

		// -- Check
		assert!(
			matches!(res, Err(Error::EntityNotFound { table: "task", id: fx_id })),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
