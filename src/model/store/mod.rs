// region:    --- Modules

use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;
