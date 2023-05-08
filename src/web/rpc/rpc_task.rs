use crate::ctx::Ctx;
use crate::model::task::{Task, TaskBmc, TaskForCreate, TaskForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForUpdate, ParamsIded};
use crate::web::Result;
use tracing::debug;

// region:    --- REST Handlers
pub async fn create_task(
	mm: ModelManager,
	ctx: Ctx,
	task_fc: TaskForCreate,
) -> Result<Task> {
	debug!("{:<12} - create_task", "HANDLER");

	let id = TaskBmc::create(&ctx, &mm, task_fc).await?;
	let task = TaskBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}

pub async fn list_tasks(mm: ModelManager, ctx: Ctx) -> Result<Vec<Task>> {
	debug!("{:<12} - create_task", "HANDLER");

	let tasks = TaskBmc::list(&ctx, &mm, None).await?;

	Ok(tasks)
}

pub async fn update_task(
	mm: ModelManager,
	ctx: Ctx,
	params: ParamsForUpdate<TaskForUpdate>,
) -> Result<Task> {
	debug!("{:<12} - update_task", "HANDLER");

	let ParamsForUpdate { id, data: task_fu } = params;
	TaskBmc::update(&ctx, &mm, id, task_fu).await?;

	let task = TaskBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}

pub async fn delete_task(
	mm: ModelManager,
	ctx: Ctx,
	params: ParamsIded,
) -> Result<Task> {
	debug!("{:<12} - delete_task", "HANDLER");

	let ParamsIded { id } = params;

	let task = TaskBmc::get(&ctx, &mm, id).await?;
	TaskBmc::delete(&ctx, &mm, id).await?;

	Ok(task)
}
// endregion: --- REST Handlers
