use crate::crypt::pwd::{self};
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::store::db::{db_get, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use crate::utils;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::{Fields, HasFields};
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

// region:    --- User Types
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub username: String,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- Timestamps
	pub cid: i64,
	pub ctime: OffsetDateTime,
	pub mid: i64,
	pub mtime: OffsetDateTime,

	// -- pwd and salts
	pub pwd: String,      // encrypted, #_scheme_id_#....
	pub pwd_salt: Uuid,   // UUID
	pub token_salt: Uuid, // UUID
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
	pub cid: i64,
	pub ctime: OffsetDateTime,
	pub mid: i64,
	pub mtime: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
}
// endregion: --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
}

impl UserBmc {
	#[allow(unused)]
	pub async fn create(
		mm: &ModelManager,
		ctx: &Ctx,
		user_fc: UserForCreate,
	) -> Result<i64> {
		let db = mm.db();

		let now = utils::now_utc();

		let user = UserForInsert {
			username: user_fc.username,
			cid: ctx.user_id(),
			ctime: now,
			mid: ctx.user_id(),
			mtime: now,
		};

		let (id, pwd_salt) = sqlb::insert()
			.table(Self::TABLE)
			.data(user.fields())
			.returning(&["id", "pwd_salt"])
			.fetch_one::<(i64, Uuid), _>(db)
			.await?;

		Self::update_pwd(mm, ctx, id, &user_fc.pwd_clear);

		Ok(id)
	}

	#[allow(unused)]
	pub async fn get(mm: &ModelManager, _ctx: &Ctx, id: i64) -> Result<User> {
		db_get::<Self, _>(mm.db(), id).await
	}

	#[allow(unused)] // For now, for test only.
	pub async fn get_for_auth_by_id(
		mm: &ModelManager,
		_ctx: &Ctx,
		id: i64,
	) -> Result<UserForAuth> {
		db_get::<Self, _>(mm.db(), id).await
	}

	pub async fn get_for_auth_by_username(
		mm: &ModelManager,
		_ctx: &Ctx,
		username: &str,
	) -> Result<Option<UserForAuth>> {
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional::<UserForAuth, _>(db)
			.await?;

		Ok(user)
	}

	pub async fn update_pwd(
		mm: &ModelManager,
		ctx: &Ctx,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		let db = mm.db();

		let user = Self::get_for_auth_by_id(mm, ctx, id).await?;

		let pwd = pwd::encrypt_pwd(&EncryptContent {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear.to_string(),
		})?;

		sqlb::update()
			.table(Self::TABLE)
			.and_where("id", "=", id)
			.data(vec![("pwd", pwd.to_string()).into()])
			.exec(db)
			.await?;

		Ok(())
	}
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use crate::test_utils;
	use anyhow::{Context, Result};

	#[tokio::test]
	async fn test_model_user_get_demo1() -> Result<()> {
		let mm = test_utils::init_dev_all().await;

		let user = UserBmc::get_for_auth_by_username(&mm, &Ctx::root_ctx(), "demo1")
			.await?
			.context("Should have user 'demo1'")?;

		assert_eq!("demo1", user.username);
		Ok(())
	}

	#[tokio::test]
	async fn test_model_user_create_demo2() -> Result<()> {
		// -- Setup & Fixtures
		let mm = test_utils::init_dev_all().await;
		let ctx = Ctx::root_ctx();
		let username = "demo2";
		let pwd_clear = "wecome2";

		// -- Exec
		let id = UserBmc::create(
			&mm,
			&ctx,
			UserForCreate {
				username: username.to_string(),
				pwd_clear: pwd_clear.to_string(),
			},
		)
		.await?;

		// -- Check - username
		let user = UserBmc::get_for_auth_by_id(&mm, &ctx, id).await?;
		assert_eq!("demo2", user.username);

		// -- Check - pwd
		pwd::validate_pwd(
			&EncryptContent {
				salt: user.pwd_salt.to_string(),
				content: pwd_clear.to_string(),
			},
			&user.pwd,
		)?;

		Ok(())
	}
}
// endregion: --- Tests
