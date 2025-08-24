use crate::app::App;
use crate::errors::Error;
use crate::ui::views::catalog_view::Catalog;
use crate::ui::views::update_view::Update;
use crate::ui::views::{View, start_view::Start};
use ratatui::crossterm::event::KeyEvent;

/// Handles the key events and updates the state of [`App`].
pub async fn key_handler(app: &mut App, key: KeyEvent) -> Result<(), Error> {
    match &app.view {
        View::Start(_) => Start::select(app, &key).await?,
        View::Catalog(_) => Catalog::select(app, &key).await?,
        View::Update(_) => Update::select(app, &key).await?,
    }

    Ok(())
}

pub trait Selectable {
    async fn select(app: &mut App, key: &KeyEvent) -> Result<(), Error>;
}
