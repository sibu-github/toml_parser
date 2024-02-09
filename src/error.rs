use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid string")]
    InvalidString,
}
