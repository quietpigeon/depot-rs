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
/// Gives the latest version and a short description.
pub fn search_crate(c: &str) -> Result<String, Error> {
    let stdout = Command::new("cargo")
        .arg("info")
        .arg(c)
        .output()?
        .stdout
        .apply(String::from_utf8)?;

    Ok(stdout)
}

pub async fn install_crate(c: &str) -> Result<(), Error> {
    Command::new("cargo")
        .arg("install")
        .arg(c)
        .arg("--locked")
        .output()?;
    Ok(())
}

pub async fn uninstall_crate(c: &str) -> Result<(), Error> {
    Command::new("cargo").arg("uninstall").arg(c).output()?;
    Ok(())
}
