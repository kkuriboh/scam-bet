use redis::Client;

use super::DatabaseError;

pub type Pool = r2d2::Pool<Client>;

pub async fn connect(uri: &str) -> Result<Pool, DatabaseError> {
    r2d2::Pool::new(Client::open(uri).map_err(|_| DatabaseError::ConnectionFailed)?)
        .map_err(|_| DatabaseError::ConnectionFailed)
}
