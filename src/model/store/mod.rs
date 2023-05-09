// region:    --- Modules

pub(in crate::model) mod init_dev_db;

use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;
