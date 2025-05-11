use crate::depot::DepotState;
use crate::errors::Error;
use ratatui::Frame;
use ratatui::style::{Color, Style};
use views::View;
use views::catalog_view::Catalog;
use views::start_view::Start;

mod banner;
pub mod views;

const DEFAULT_STYLE: Style = Style::new().fg(Color::Yellow);
const HIGHLIGHT_STYLE: Style = Style::new().bg(Color::Black);

/// Renders the user interface.
pub fn render(view: &mut View, state: &mut DepotState, frame: &mut Frame) -> Result<(), Error> {
    match view {
        View::StartView(_) => Start::render(state, frame)?,
        View::CatalogView(_) => Catalog::render(state, frame)?,
    }

    Ok(())
}

pub trait Drawable {
    fn render(state: &mut DepotState, frame: &mut Frame) -> Result<(), Error>;
}
