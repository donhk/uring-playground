use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryErr {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Not found")]
    NotFound,
}
