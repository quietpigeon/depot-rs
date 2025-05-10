use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to run command")]
    Cli(#[from] std::io::Error),

    #[error("failed to run command")]
    FromUtf8(#[from] FromUtf8Error),
}
