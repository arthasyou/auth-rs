pub mod error_code;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("service error: {0}")]
    ServiceError(#[from] service_utils_rs::error::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("db error: {0}")]
    DbError(#[from] surrealdb::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
