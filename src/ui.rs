use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph};

mod banner;

/// Renders the user interface.
pub fn render(frame: &mut Frame) {
    let text = banner::BANNER;
    frame.render_widget(
        Paragraph::new(text)
            .style(Style::new().fg(Color::Yellow))
            .block(Block::bordered())
            .centered(),
        frame.area(),
    )
}
