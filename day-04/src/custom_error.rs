use derive_more::From;
use miette::Diagnostic;

use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, From)]
pub enum CardError {
    Number,
    Parts,
}

impl From<CardError> for AocError {
    fn from(err: CardError) -> Self {
        AocError::from(err)
    }
}
