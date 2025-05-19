use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to run command")]
    Cli(#[from] std::io::Error),

    #[error("failed to run command")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("failed to parse cargo command stdout: {0}")]
    Parser(nom::Err<nom::error::Error<String>>),

    #[error("failed to create text")]
    DisplayFmt(#[from] std::fmt::Error),
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Parser(err.map_input(|input| input.into()))
    }
}
