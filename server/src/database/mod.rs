use thiserror::Error;

pub mod cache;
pub mod main_db;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to connect to the database")]
    ConnectionFailed,
}
