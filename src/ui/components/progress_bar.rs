use crate::{depot::DepotState, errors::Error, ui::DEFAULT_STYLE};
use ratatui::style::Style;
use ratatui::symbols::line::THICK;
use ratatui::widgets::{Block, LineGauge, Padding};

pub fn new(state: &mut DepotState) -> Result<LineGauge, Error> {
    let line_gauge_title = if state.synced {
        "done!"
    } else {
        "fetching crates..."
    };

    let l = LineGauge::default()
        .block(
            Block::new()
                .padding(Padding::horizontal(60))
                .title(line_gauge_title)
                .style(DEFAULT_STYLE)
                .title_alignment(ratatui::layout::Alignment::Center),
        )
        .label("")
        .line_set(THICK)
        .filled_style(Style::new().fg(ratatui::style::Color::Magenta))
        .ratio(state.info_state);

    Ok(l)
}
