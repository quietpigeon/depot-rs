use crate::depot::DepotState;
use crate::errors::Error;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph};

mod banner;

/// Renders the user interface.
pub fn render(state: &mut DepotState, frame: &mut Frame) -> Result<(), Error> {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Fill(1)])
        .split(frame.area());
    let banner = Text::raw(banner::BANNER);
    let _ = state.sync()?;

    let banner_area = center(
        layout[0],
        Constraint::Length(banner.width() as u16),
        Constraint::Length(banner.height() as u16),
    );

    // Main area.
    frame.render_widget(
        Block::bordered().style(Style::new().fg(Color::Yellow)),
        frame.area(),
    );
    frame.render_widget(
        Paragraph::new(banner)
            .style(Style::new().fg(Color::Yellow))
            .centered(),
        banner_area,
    );
    frame.render_widget(
        Paragraph::new(format!(
            "You have {} crates installed.",
            state.depot.crate_count
        ))
        .style(Style::new().fg(Color::Yellow))
        .centered(),
        layout[1],
    );

    Ok(())
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
