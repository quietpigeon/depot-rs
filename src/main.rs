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
    let tick_rate = std::time::Duration::from_millis(100);
    let result = app::App::new().run(terminal, tick_rate).await;
    ratatui::restore();

    Ok(result?)
}
