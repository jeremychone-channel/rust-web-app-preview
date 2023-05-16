use crate::crypt::pwd::{self};
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::Fields;
use sqlx::postgres::PgRow;
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

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,

	// -- pwd and token salts
	pub pwd: Option<String>, // encrypted, #_scheme_id_#....
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- pwd and salts
	pub token_salt: Uuid,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
}

/// Marker trait
pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

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
		let UserForCreate { username, pwd_clear } = user_fc;

		let user_fi = UserForInsert { username: username.to_string() };

		let user_id = base::create::<Self, _>(ctx, mm, user_fi)
			.await
			.map_err(|model_error| match model_error {
				Error::Sqlx(sqlx_error) => {
					if let Some((code, constraint)) = sqlx_error
						.as_database_error()
						.and_then(|db_error| {
							db_error.code().zip(db_error.constraint())
						}) {
						// "23505" => postgresql "unique violation"
						// Note: Here we could part the
						if code == "23505" && constraint == "user_username_key" {
							return Error::UserAlreadyExists { username };
						}
					}
					Error::Sqlx(sqlx_error)
				}
				_ => model_error,
			})?;

		Self::update_pwd(ctx, mm, user_id, &pwd_clear).await?;

		Ok(user_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional::<_, E>(db)
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

		let user: UserForLogin = Self::get(ctx, mm, id).await?;

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
	use crate::crypt::pwd::validate_pwd;
	use anyhow::{Context, Result};

	#[tokio::test]
	async fn test_get_demo1() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "demo1";

		// -- Exec
		let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
			.await?
			.context("Should have user 'demo1'")?;

		// -- Check
		assert_eq!(fx_username, fx_username);

		Ok(())
	}

	#[tokio::test]
	async fn test_pwd_demo1() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "demo1";
		let fx_pwd = "welcome";

		// -- Check
		let user: UserForLogin = UserBmc::first_by_username(&ctx, &mm, fx_username)
			.await?
			.context("Should have user 'demo1'")?;
		validate_pwd(
			&EncryptContent {
				content: fx_pwd.to_string(),
				salt: user.pwd_salt.to_string(),
			},
			&user.pwd.unwrap(),
		)?;

		Ok(())
	}

	#[tokio::test]
	async fn test_create_demo2() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "demo2";
		let fx_pwd_clear = "wecome2";

		// -- Exec
		let id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: fx_pwd_clear.to_string(),
			},
		)
		.await?;

		// -- Check - username
		let user: UserForLogin = UserBmc::get(&ctx, &mm, id).await?;
		assert_eq!(user.username, fx_username);

		// -- Check - pwd
		pwd::validate_pwd(
			&EncryptContent {
				salt: user.pwd_salt.to_string(),
				content: fx_pwd_clear.to_string(),
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
		let fx_username = "demo3";
		let fx_pwd_clear = "welcome3";

		// -- Exec
		let id = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: fx_pwd_clear.to_string(),
			},
		)
		.await?;

		let res = UserBmc::create(
			&ctx,
			&mm,
			UserForCreate {
				username: fx_username.to_string(),
				pwd_clear: fx_pwd_clear.to_string(),
			},
		)
		.await;

		let expected_error: Result<i64, Error> =
			Err(Error::UserAlreadyExists { username: fx_username.to_string() });

		assert!(
			matches!(&res, expected_error),
			"Error not matching.\nexpected: {expected_error:?}\nfound: {res:?}"
		);

		Ok(())
	}
}
// endregion: --- Tests
