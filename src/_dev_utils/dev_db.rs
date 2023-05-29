use crate::crypt::pwd::{self};
use crate::crypt::EncryptContent;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;
use uuid::Uuid;

type Db = Pool<Postgres>;

// NOTE: Harcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	// -- Create the app_db/app_user with posgres user.
	{
		let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
		pexec(&root_db, SQL_RECREATE_DB).await?;
	}

	// -- Get sql files.
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|e| e.ok().map(|e| e.path()))
		.collect();
	paths.sort();

	// -- SQL Execute each file.
	let app_db = new_db_pool(PG_DEV_APP_URL).await?;
	for path in paths {
		if let Some(path) = path.to_str() {
			let path = path.replace('\\', "/"); // for Windows.

			// Only take .sql and skip the SQL_RECREATE
			if path.ends_with(".sql") && path != SQL_RECREATE_DB {
				pexec(&app_db, &path).await?;
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
	let content = fs::read_to_string(file)?;

	// TODO: Make the split more sql proof.
	let sqls: Vec<&str> = content.split(';').collect();

	// -- SQL Execute each part.
	let mut fn_sql_parts: Vec<&str> = Vec::new();
	for sql in sqls {
		// -- Trick to not split function body
		//    (TODO: Needs to be make it more robust.)

		// FIXME: This works for simple sql files with trigger with $$ notation.
		//        However, will probably break for other specific cases.
		//        It needs to be made more robust.
		//        sqlx does not seems to have a non static file executor.
		// If it is the begin of a function we start keeping track
		if sql.contains("BEGIN") {
			fn_sql_parts.push(sql);
		} else if !fn_sql_parts.is_empty() {
			fn_sql_parts.push(sql);

			// Here we assume the end will be `$$ LANGUAGE plpgsql;`
			if sql.trim().starts_with("$$") {
				let sql = format!("{};", fn_sql_parts.join(";"));
				sqlx::query(&sql).execute(db).await?;
				fn_sql_parts.clear();
			}
		} else {
			sqlx::query(sql).execute(db).await?;
		}
	}

	Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_con_url)
		.await
}
