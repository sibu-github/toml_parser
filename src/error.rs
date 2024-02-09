use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Illegal character {0} at line {1}:{2}")]
    IllegalChar(char, usize, usize),
}
