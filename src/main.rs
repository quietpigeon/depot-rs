use crate::errors::Error;
mod app;
mod commands;
mod depot;
mod errors;
mod events;
mod keys;
mod parser;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal).await;
    ratatui::restore();

    Ok(result?)
}
