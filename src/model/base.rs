//! Base Bmcs implementations.
//! For now, focuses on the "Db Bmcs."

use crate::ctx::Ctx;
use crate::model::{Error, ModelManager, Result};
use crate::utils;
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub trait DbBmc {
	const TABLE: &'static str;
	const HAS_TIMESTAMPS: bool;
}

pub async fn create<MC, D>(ctx: &Ctx, mm: &ModelManager, data: D) -> Result<i64>
where
	MC: DbBmc,
	D: HasFields,
{
	let db = mm.db();

	let mut fields = data.fields();

	if MC::HAS_TIMESTAMPS {
		let user_id = ctx.user_id();
		let now = utils::now_utc();
		fields.push(("cid", user_id).into());
		fields.push(("ctime", now).into());
		fields.push(("mid", user_id).into());
		fields.push(("mtime", now).into());
	}

	let (id,) = sqlb::insert()
		.table(MC::TABLE)
		.data(fields)
		.returning(&["id"])
		.fetch_one::<_, (i64,)>(db)
		.await?;

	Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();

	let entity = sqlb::select()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.fetch_one::<_, E>(db)
		.await
		.map_err(|ex| match ex {
			sqlx::Error::RowNotFound => {
				Error::EntityNotFound { table: MC::TABLE, id }
			}
			_ => Error::Sqlx(ex),
		})?;

	Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();

	let entity = sqlb::select()
		.table(MC::TABLE)
		.fetch_all::<_, E>(db)
		.await?;

	Ok(entity)
}

pub async fn update<MC, D>(
	ctx: &Ctx,
	mm: &ModelManager,
	id: i64,
	data: D,
) -> Result<()>
where
	MC: DbBmc,
	D: HasFields,
{
	let db = mm.db();

	let mut fields = data.fields();

	if MC::HAS_TIMESTAMPS {
		let user_id = ctx.user_id();
		let now = utils::now_utc();
		fields.push(("mid", user_id).into());
		fields.push(("mtime", now).into());
	}

	let count = sqlb::update()
		.table(MC::TABLE)
		.data(fields)
		.and_where("id", "=", id)
		.exec(db)
		.await?;

	if count == 0 {
		Err(Error::EntityNotFound { table: MC::TABLE, id })
	} else {
		Ok(())
	}
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let db = mm.db();

	let count = sqlb::delete()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.exec(db)
		.await?;

	if count == 0 {
		Err(Error::EntityNotFound { table: MC::TABLE, id })
	} else {
		Ok(())
	}
}
