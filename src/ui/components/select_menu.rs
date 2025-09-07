use crate::errors::Error;
use crate::ui::{DEFAULT_SECONDARY_COLOR, DEFAULT_STYLE};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn new() -> Result<Paragraph<'static>, Error> {
    let crates = Line::from(vec![
        Span::styled(" View ", DEFAULT_STYLE),
        Span::styled(
            "c",
            Style::default()
                .fg(DEFAULT_SECONDARY_COLOR)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled("rates", DEFAULT_STYLE),
    ]);

    let updates = Line::from(vec![
        Span::styled("󰚰 ", DEFAULT_STYLE),
        Span::styled(
            "U",
            Style::default()
                .fg(DEFAULT_SECONDARY_COLOR)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled("pdate crates", DEFAULT_STYLE),
    ]);

    let lines = vec![crates, updates];
    let p = Paragraph::new(lines).centered();

    Ok(p)
}
