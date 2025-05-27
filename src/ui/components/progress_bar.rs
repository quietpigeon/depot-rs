use crate::{errors::Error, ui::DEFAULT_STYLE};
use ratatui::widgets::Paragraph;

pub fn new() -> Result<Paragraph<'static>, Error> {
    let text = "fetching crates...";
    let p = Paragraph::new(text)
        .style(DEFAULT_STYLE)
        .alignment(ratatui::layout::Alignment::Center);

    Ok(p)
}
