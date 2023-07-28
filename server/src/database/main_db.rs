use scylla::{Session, SessionBuilder};

use super::DatabaseError;

pub type Pool = Session;

pub async fn connect(uri: &str) -> Result<Session, DatabaseError> {
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .map_err(|_| DatabaseError::ConnectionFailed)
}
