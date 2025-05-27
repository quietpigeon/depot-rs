use crate::{errors::Error, ui::DEFAULT_STYLE};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

pub fn new() -> Result<Paragraph<'static>, Error> {
    let lines = Line::from(vec![
        Span::styled(
            "C",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled("rates", DEFAULT_STYLE),
    ]);

    let text = Text::from(lines);
    let p = Paragraph::new(text).centered();

    Ok(p)
}
