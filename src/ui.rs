use crate::depot::DepotState;
use crate::errors::Error;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
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
        View::Start(_) => Start::render(state, frame)?,
        View::Catalog(_) => Catalog::render(state, frame)?,
        View::Update(_) => Update::render(state, frame)?,
    }

    Ok(())
}

/// Renders a helpline.
pub fn render_helpline(frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let line = Line::from(vec![
        Span::raw("Press "),
        Span::raw("?").style(Style::new().fg(DEFAULT_SECONDARY_COLOR)),
        Span::raw(" for help, "),
        Span::raw("q").style(Style::new().fg(DEFAULT_SECONDARY_COLOR)),
        Span::raw(" to exit"),
    ]);

    let footer_bar = Paragraph::new(line);
    frame.render_widget(footer_bar, area);

    Ok(())
}

pub trait Drawable {
    fn render(state: &mut DepotState, frame: &mut Frame) -> Result<(), Error>;
}
