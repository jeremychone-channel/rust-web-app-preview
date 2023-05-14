use crate::crypt::pwd::{self};
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::base::{get, DbBmc};
use crate::model::{Error, ModelManager, Result};
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

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
	pub cid: i64,
	pub ctime: OffsetDateTime,
	pub mid: i64,
	pub mtime: OffsetDateTime,
}

#[serde_as]
#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- pwd and salts
	pub pwd: Option<String>, // encrypted, #_scheme_id_#....
	pub pwd_salt: Uuid,      // UUID
	pub token_salt: Uuid,    // UUID
}

// endregion: --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
	const HAS_TIMESTAMPS: bool = true;
}

impl UserBmc {
	#[allow(unused)]
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_fc: UserForCreate,
	) -> Result<i64> {
		let db = mm.db();

		let now = utils::now_utc();

		let UserForCreate { username, pwd_clear } = user_fc;

		let user = UserForInsert {
			username: username.to_string(),
			cid: ctx.user_id(),
			ctime: now,
			mid: ctx.user_id(),
			mtime: now,
		};

		let (id, pwd_salt) = sqlb::insert()
			.table(Self::TABLE)
			.data(user.fields())
			.returning(&["id", "pwd_salt"])
			.fetch_one::<_, (i64, Uuid)>(db)
			.await
			.map_err(|sqlx_error| match sqlx_error.as_database_error() {
				Some(db_error) => {
					if let Some(code) = db_error.code() {
						if code == "23505" {
							return Error::UserAlreadyExists { username };
						}
					}
					Error::Sqlx(sqlx_error)
				}
				_ => Error::Sqlx(sqlx_error),
			})?;

		Self::update_pwd(ctx, mm, id, &pwd_clear).await?;

		Ok(id)
	}

	#[allow(unused)]
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<User> {
		get::<Self, _>(ctx, mm, id).await
	}

	pub async fn get_for_auth_by_id(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<UserForAuth> {
		get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_for_auth_by_username(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<UserForAuth>> {
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional::<_, UserForAuth>(db)
			.await?;

		Ok(user)
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		let db = mm.db();

		let user = Self::get_for_auth_by_id(ctx, mm, id).await?;

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
	use crate::_dev_utils;
	use anyhow::{Context, Result};

	#[tokio::test]
	async fn test_get_demo1() -> Result<()> {
		let mm = _dev_utils::init_test().await;

		let user =
			UserBmc::first_for_auth_by_username(&Ctx::root_ctx(), &mm, "demo1")
				.await?
				.context("Should have user 'demo1'")?;

		assert_eq!("demo1", user.username);
		Ok(())
	}

	#[tokio::test]
	async fn test_create_demo2() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let username = "demo2";
		let pwd_clear = "wecome2";

		// -- Exec
		let id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: username.to_string(),
				pwd_clear: pwd_clear.to_string(),
			},
		)
		.await?;

		// -- Check - username
		let user = UserBmc::get_for_auth_by_id(&ctx, &mm, id).await?;
		assert_eq!("demo2", user.username);

		// -- Check - pwd
		pwd::validate_pwd(
			&EncryptContent {
				salt: user.pwd_salt.to_string(),
				content: pwd_clear.to_string(),
			},
			&user.pwd.unwrap(),
		)?;

		Ok(())
	}

	#[tokio::test]
	async fn test_create_demo3_twice_and_err() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let username = "demo3";
		let pwd_clear = "wecome3";

		// -- Exec
		let id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: username.to_string(),
				pwd_clear: pwd_clear.to_string(),
			},
		)
		.await?;

		let res = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: username.to_string(),
				pwd_clear: pwd_clear.to_string(),
			},
		)
		.await;

		let expected_error: Result<i64, Error> =
			Err(Error::UserAlreadyExists { username: username.to_string() });

		assert!(
			matches!(&res, expected_error),
			"Error not matching.\nexpected: {expected_error:?}\nfound: {res:?}"
		);

		Ok(())
	}
}
// endregion: --- Tests
