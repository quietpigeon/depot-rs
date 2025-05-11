use crate::depot::DepotState;
use crate::errors::Error;
use ratatui::Frame;
use views::View;
use views::start_view::Start;

mod banner;
pub mod views;

/// Renders the user interface.
pub fn render(view: &mut View, state: &mut DepotState, frame: &mut Frame) -> Result<(), Error> {
    match view {
        View::StartView(_) => Start::render(state, frame)?,
        View::CatalogView(_) => todo!(),
    }

    Ok(())
}

pub trait Drawable {
    fn render(state: &mut DepotState, frame: &mut Frame) -> Result<(), Error>;
}
