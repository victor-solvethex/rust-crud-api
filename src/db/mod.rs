use self::todo::TodoModel;
use crate::models::{
    db::{DBCon, DBPool},
    error::Error::{self, *},
};
use mobc::Pool;
use mobc_postgres::{
    tokio_postgres::{self},
    PgConnectionManager,
};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};

pub mod todo;

type Result<T> = std::result::Result<T, Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";

#[derive(Clone)]
pub struct DB {
    pub todo: TodoModel,
}

impl DB {
    pub async fn connect() -> Result<Self> {
        let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;

        let manager = PgConnectionManager::new(config, NoTls);
        let db_pool = Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager);
        
        DB::init_db(&db_pool).await?;

        Ok(Self {
            todo: TodoModel::new(db_pool.clone()),
        })
    }

    async fn init_db(db_pool: &DBPool) -> Result<()> {
        let init_file = fs::read_to_string(INIT_SQL)?;
        let con: DBCon = db_pool.get().await.map_err(Error::DBPoolError)?;
        con.batch_execute(init_file.as_str())
            .await
            .map_err(DBInitError)?;
        Ok(())
    }
}
