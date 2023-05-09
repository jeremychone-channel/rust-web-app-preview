//! Base Bmcs implementations.
//! For now, focuses on the "Db Bmcs."

// region:    --- Modules

use crate::ctx::Ctx;
use crate::model::store::Db;
use crate::model::{Error, ModelManager, Result};
use crate::utils;
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

// endregion: --- Modules

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn db_create<MC, D>(
	_ctx: &Ctx,
	mm: &ModelManager,
	user_id: Option<i64>,
	data: D,
) -> Result<i64>
where
	MC: DbBmc,
	D: HasFields,
{
	let db = mm.db();

	let mut fields = data.fields();
	if let Some(user_id) = user_id {
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
		.fetch_one::<(i64,), _>(db)
		.await?;

	Ok(id)
}

pub async fn db_get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();

	let entity = sqlb::select()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.fetch_one::<E, _>(db)
		.await
		.map_err(|ex| match ex {
			sqlx::Error::RowNotFound => {
				Error::EntityNotFound { entity: MC::TABLE, id }
			}
			_ => Error::Sqlx(ex),
		})?;

	Ok(entity)
}

pub async fn db_list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();

	let entity = sqlb::select()
		.table(MC::TABLE)
		.fetch_all::<E, _>(db)
		.await?;

	Ok(entity)
}

pub async fn db_update<MC, D>(
	_ctx: &Ctx,
	mm: &ModelManager,
	user_id: Option<i64>,
	id: i64,
	data: D,
) -> Result<()>
where
	MC: DbBmc,
	D: HasFields,
{
	let db = mm.db();

	let mut fields = data.fields();
	if let Some(user_id) = user_id {
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
		Err(Error::EntityNotFound { entity: MC::TABLE, id })
	} else {
		Ok(())
	}
}

pub async fn db_delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let db = mm.db();

	let count = sqlb::delete()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.returning(&["id"])
		.exec(db)
		.await?;

	if count == 0 {
		Err(Error::EntityNotFound { entity: MC::TABLE, id })
	} else {
		Ok(())
	}
}
