use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("An error occurred while parsing the date: {0}")]
    DateParseError(String),

    #[error("Invalid currency provided: {0}")]
    InvalidCurrency(String),

    #[error("Network error occurred: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("An unexpected error occurred: {0}")]
    UnexpectedError(String),
}
