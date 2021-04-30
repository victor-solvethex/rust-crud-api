use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use crate::models::error::Error;
use async_trait::async_trait;

pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait DbModel {
    fn get_pool(&self) -> DBPool;

    async fn get_db_con(&self) -> Result<DBCon> {
        self.get_pool().get().await.map_err(Error::DBPoolError)
    }
}
