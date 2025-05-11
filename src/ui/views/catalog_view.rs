use super::{View, start_view::Start};
use crate::ui::{DEFAULT_STYLE, HIGHLIGHT_STYLE};
use crate::{depot::DepotState, errors::Error, keys::Selectable, ui::Drawable};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, List, ListItem};

#[derive(Debug)]
pub struct Catalog;

impl Drawable for Catalog {
    fn render(
        state: &mut crate::depot::DepotState,
        frame: &mut ratatui::Frame,
    ) -> Result<(), crate::errors::Error> {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Fill(1)])
            .split(frame.area());
        let left = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Fill(1)])
            .split(layout[0]);
        let krates: Vec<ListItem> = state
            .depot
            .store
            .0
            .iter()
            .map(|krate| ListItem::from(krate.name.clone()).fg(Color::Yellow))
            .collect();
        let list = List::new(krates)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Installed crates")
                    .style(DEFAULT_STYLE),
            )
            .highlight_symbol("* ")
            .highlight_style(HIGHLIGHT_STYLE)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(list, left[0], &mut state.list_state);

        Ok(())
    }
}

impl Selectable for Catalog {
    fn select(
        app: &mut crate::app::App,
        key: &crossterm::event::KeyEvent,
    ) -> Result<(), crate::errors::Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => app.view = View::StartView(Start),
            (_, KeyCode::Char('j')) => select_next(&mut app.state)?,
            (_, KeyCode::Char('k')) => select_previous(&mut app.state)?,
            _ => {}
        }
        Ok(())
    }
}

fn select_next(state: &mut DepotState) -> Result<(), Error> {
    Ok(state.list_state.select_next())
}

fn select_previous(state: &mut DepotState) -> Result<(), Error> {
    Ok(state.list_state.select_previous())
}
