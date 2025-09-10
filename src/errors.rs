use std::{fmt, string::FromUtf8Error};
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
    #[error("unexpected error occured for: {0}")]
    Unexpected(String),
    #[error("operation failed: {0}")]
    HandleKrate(ChannelError),
    #[error("failed to receive event")]
    ReceiveEvent,
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Parser(err.map_input(|input| input.into()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelError {
    UpdateKrate,
    UninstallKrate,
    KrateInfo,
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelError::UpdateKrate => write!(f, "failed to update krate"),
            ChannelError::UninstallKrate => write!(f, "failed to uninstall krate"),
            ChannelError::KrateInfo => write!(f, "failed to fetch krate"),
        }
    }
}
