use crate::errors::Error;
use apply::Apply;
use std::process::Command;

/// Lists out all of the installed crates.
pub fn list_crates() -> Result<String, Error> {
    let stdout = Command::new("cargo")
        .args(["install", "--list"])
        .output()?
        .stdout
        .apply(String::from_utf8)?;

    Ok(stdout)
}

/// Searches for a specific crate on crates.io.
/// Gives latest version and short description.
pub fn search_crate(c: &str) -> Result<String, Error> {
    let stdout = Command::new("cargo")
        .arg("search")
        .arg(c)
        .output()?
        .stdout
        .apply(String::from_utf8)?;

    Ok(stdout)
}
