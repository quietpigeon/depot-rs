use crate::{errors::Error, ui::DEFAULT_STYLE};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn new() -> Result<Paragraph<'static>, Error> {
    let crates = Line::from(vec![
        Span::styled("View ", DEFAULT_STYLE),
        Span::styled(
            "c",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled("rates", DEFAULT_STYLE),
    ]);

    let updates = Line::from(vec![
        Span::styled(
            "U",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled("pdate crates", DEFAULT_STYLE),
    ]);

    let lines = vec![crates, updates];
    let p = Paragraph::new(lines).centered();

    Ok(p)
}
