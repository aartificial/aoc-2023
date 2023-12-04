use derive_more::From;
use miette::Diagnostic;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, From)]
pub enum GameError {
    Number(ParseIntError),
    InvalidColor(String),
    Color,
}
