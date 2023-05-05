use crate::model::{Error, Result};
use crate::utils;
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Pool, Postgres};

pub type Db = Pool<Postgres>;

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn db_create<MC, D>(db: &Db, user_id: Option<i64>, data: D) -> Result<i64>
where
	MC: DbBmc,
	D: HasFields,
{
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

pub async fn db_get<MC, E>(db: &Db, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let entity = sqlb::select()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.fetch_one::<E, _>(db)
		.await
		.map_err(|ex| match ex {
			sqlx::Error::RowNotFound => Error::EntityNotFound { typ: MC::TABLE, id },
			_ => Error::Sqlx(ex),
		})?;

	Ok(entity)
}

pub async fn db_list<MC, E>(db: &Db) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let entity = sqlb::select()
		.table(MC::TABLE)
		.fetch_all::<E, _>(db)
		.await?;

	Ok(entity)
}

pub async fn db_update<MC, D>(
	db: &Db,
	user_id: Option<i64>,
	id: i64,
	data: D,
) -> Result<()>
where
	MC: DbBmc,
	D: HasFields,
{
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
		Err(Error::EntityNotFound { typ: MC::TABLE, id })
	} else {
		Ok(())
	}
}

pub async fn db_delete<MC>(db: &Db, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let count = sqlb::delete()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.returning(&["id"])
		.exec(db)
		.await?;

	if count == 0 {
		Err(Error::EntityNotFound { typ: MC::TABLE, id })
	} else {
		Ok(())
	}
}