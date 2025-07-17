use crate::{errors::Error, ui::DEFAULT_STYLE};
use ratatui::widgets::Paragraph;
use throbber_widgets_tui::{Throbber, ThrobberState};

pub fn new(throbber_state: &ThrobberState) -> Result<Paragraph<'static>, Error> {
    let throbber = Throbber::default()
        .label("fetching crates")
        .style(DEFAULT_STYLE)
        .to_line(throbber_state);
    let text = Paragraph::new(throbber).centered();

    Ok(text)
}
