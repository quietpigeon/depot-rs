use crate::{
    app::App,
    errors::Error,
    ui::views::{View, start_view::Start},
};
use crossterm::event::KeyEvent;

/// Handles the key events and updates the state of [`App`].
pub fn key_handler(app: &mut App, key: KeyEvent) -> Result<(), Error> {
    match &app.view {
        View::StartView(_) => Start::select(app, &key)?,
        View::CatalogView(_) => todo!(),
    }

    Ok(())
}

pub trait Selectable {
    fn select(app: &mut App, key: &KeyEvent) -> Result<(), Error>;
}
