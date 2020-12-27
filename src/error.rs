use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, ThisError)]
pub enum Error {
    #[error("An error ocurred during database interaction. {0}")]
    DatabaseError(String),
}

impl From<SqlxError> for Error {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => Error::DatabaseError(db_error.to_string()),
            None => {
                error!("{:?}", sqlx_error);
                Error::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}
