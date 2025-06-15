mod app;
mod commands;
mod depot;
mod errors;
mod keys;
mod parser;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal).await;
    ratatui::restore();
    result
}
