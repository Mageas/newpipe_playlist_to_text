use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewpipedError {
    #[error("Unable to connect to '{1}' : {0}")]
    DatabaseConnection(#[source] rusqlite::Error, String),

    #[error("Unable to prepare the statement for '{1}' : {0}")]
    DatabasePrepareStatement(#[source] rusqlite::Error, String),

    #[error("Unable to query '{1}' : {0}")]
    DatabaseQuery(#[source] rusqlite::Error, String),
}

pub type NewpipedResult<T = ()> = Result<T, NewpipedError>;
