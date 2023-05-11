use crate::crypt::pwd::{self};
use crate::crypt::EncryptContent;
use crate::model;
use crate::model::store::Db;
use sqlx::postgres::PgPoolOptions;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tracing::{error, info};
use uuid::Uuid;

// TODO: Probably need to move this to crate::test_utils

// NOTE: As this module is supposed to be ran only on local development
//       we hardcode the postgres "root" url/pwd and the app_db names and pwd
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

// sql files
const SQL_DIR: &str = "sql/initial";
const SQL_RECREATE: &str = "sql/initial/00-recreate-db.sql";

// type Db = Pool<Postgres>;

pub async fn init_dev_db() -> Result<(), model::Error> {
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	// Prevent to call this function more than one time to ensure
	// unicity is insured upstream.
	// TODO: Might want to reassess this strategy.
	static INIT: AtomicU64 = AtomicU64::new(0);
	let init = INIT.fetch_add(1, Ordering::Relaxed);
	if init > 0 {
		panic!("Cannot call model::store::init_dev_db twice.");
	}

	// -- Create the db with PG_ROOT (dev only).
	{
		let root_db: Db = new_db_pool(PG_DEV_POSTGRES_URL, 1).await?;
		pexec(&root_db, SQL_RECREATE).await?;
	}

	// -- Run the app sql files.
	let app_db = new_db_pool(PG_DEV_APP_URL, 1).await?;
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|e| e.ok().map(|e| e.path()))
		.collect();
	paths.sort();

	// -- Execute each file.
	for path in paths {
		if let Some(path) = path.to_str() {
			// Only take .sql and skip the SQL_RECREATE
			if path.ends_with(".sql") && path != SQL_RECREATE {
				pexec(&app_db, path).await?;
			}
		}
	}

	// -- Set the "welcome" password to demo1
	let (id, salt): (i64, Uuid) = sqlx::query_as(
		r#"
	SELECT id, pwd_salt FROM "user"
	WHERE username = $1
	"#,
	)
	.bind("demo1")
	.fetch_one(&app_db)
	.await?;

	let salt = salt.to_string();
	let pwd = pwd::encrypt_pwd(&EncryptContent {
		salt: salt.to_string(),
		content: "welcome".to_string(),
	})?;

	sqlx::query(r#"UPDATE "user" SET pwd = $1 WHERE id = $2"#)
		.bind(pwd)
		.bind(id)
		.execute(&app_db)
		.await?;

	Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	// -- Read the file.
	let content = fs::read_to_string(file).map_err(|ex| {
		error!("ERROR reading {} (cause: {:?})", file, ex);
		ex
	})?;

	// TODO: Make the split more sql proof.
	let sqls: Vec<&str> = content.split(';').collect();

	// -- SQL Execute each part.
	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_con_url: &str, max_con: u32) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(max_con)
		.acquire_timeout(Duration::from_millis(500)) // Needs to find replacement
		.connect(db_con_url)
		.await
}
