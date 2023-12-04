use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Error while parsing: {0}")]
    NomError(nom::Err<nom::error::Error<&'static str>>),
    #[error("Other error: {0}")]
    Other(String),
}

pub type ParseResult<T> = Result<T, ParseError>;
