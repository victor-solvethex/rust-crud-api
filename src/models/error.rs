use mobc_postgres::tokio_postgres;
use serde::Serialize;
use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error getting connection from the DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("Error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("Error creating table: {0}")]
    DBInitError(tokio_postgres::Error),
    #[error("Error reding file: {0}")]
    ReadFileError(#[from] std::io::Error),
}

impl Reject for Error {}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
