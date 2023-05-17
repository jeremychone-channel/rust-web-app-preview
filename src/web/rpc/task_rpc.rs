use crate::ctx::Ctx;
use crate::model::task::{Task, TaskBmc, TaskForCreate, TaskForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_task(
	mm: ModelManager,
	ctx: Ctx,
	task_fc: TaskForCreate,
) -> Result<Task> {
	let id = TaskBmc::create(&ctx, &mm, task_fc).await?;
	let task = TaskBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}

pub async fn list_tasks(mm: ModelManager, ctx: Ctx) -> Result<Vec<Task>> {
	let tasks = TaskBmc::list(&ctx, &mm).await?;

	Ok(tasks)
}

pub async fn update_task(
	mm: ModelManager,
	ctx: Ctx,
	params: ParamsForUpdate<TaskForUpdate>,
) -> Result<Task> {
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
	let ParamsIded { id } = params;

	let task = TaskBmc::get(&ctx, &mm, id).await?;
	TaskBmc::delete(&ctx, &mm, id).await?;

	Ok(task)
}
