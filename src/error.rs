use std::num::ParseIntError;

use crate::parser::Rule;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(#[from] pest::error::Error<Rule>),
    #[error("Invalid argument '{0}': Argument value cannot exceed {1}")]
    ExceedBounds(u16, u16),
    #[error("Encountered internal error: {0}")]
    Internal(String),
    #[error("Error: {:?} expected {1} argument(s), but found {2}")]
    WrongNumArgs(Rule, usize, usize),
    #[error("Parse error: {0}")]
    NumParse(#[from] ParseIntError),
    #[error("Label '{0}' is defined twice")]
    DuplicateLabel(String),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}
