use crate::depot::DepotState;
use crate::errors::Error;
use crate::ui::views::start_view;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use views::View;
use views::catalog_view::Catalog;
use views::start_view::Start;
use views::update_view::Update;

mod banner;
mod components;
pub mod views;

const DEFAULT_PRIMARY_COLOR: Color = Color::Yellow;
const DEFAULT_SECONDARY_COLOR: Color = Color::Cyan;
const DEFAULT_STYLE: Style = Style::new().fg(DEFAULT_PRIMARY_COLOR);
const HIGHLIGHT_STYLE: Style = Style::new().bg(Color::Black);

/// Renders the user interface.
pub fn render(view: &mut View, state: &mut DepotState, frame: &mut Frame) -> Result<(), Error> {
    match view {
        View::Start(_) => Start::render(&start_view::Start, state, frame)?,
        View::Catalog(_) => Catalog::render(&Catalog, state, frame)?,
        View::Update(_) => Update::render(&Update, state, frame)?,
    }

    Ok(())
}

pub trait Drawable {
    fn render(&self, state: &mut DepotState, frame: &mut Frame) -> Result<(), Error>;

    fn render_helpline(&self, frame: &mut Frame, area: Rect) -> Result<(), Error>;
}
