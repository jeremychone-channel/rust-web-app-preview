use sqlx::{Pool, Postgres};

pub(in crate::model) mod init_dev_db;

pub type Db = Pool<Postgres>;
