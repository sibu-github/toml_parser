use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    InvalidString,
}