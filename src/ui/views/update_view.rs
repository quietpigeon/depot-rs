use super::{View, start_view::Start};
use crate::keys::Selectable;
use crate::ui::{DEFAULT_COLOR, DEFAULT_STYLE, Drawable, HIGHLIGHT_STYLE};
use crossterm::event::KeyCode;
use ratatui::layout::Margin;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, List, ListItem};

#[derive(Debug)]
pub struct Update;

impl Drawable for Update {
    fn render(
        state: &mut crate::depot::DepotState,
        frame: &mut ratatui::Frame,
    ) -> Result<(), crate::errors::Error> {
        let krates: Vec<ListItem> = state
            .depot
            .get_outdated_krates()?
            .0
            .iter()
            .map(|krate| {
                ListItem::from(format!(
                    "{}  {} -> {}",
                    krate.name.clone(),
                    krate.version.clone(),
                    krate.info.latest_version.clone()
                ))
                .fg(DEFAULT_COLOR)
            })
            .collect();
        let krate_list = List::new(krates)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Outdated crates")
                    .style(DEFAULT_STYLE),
            )
            .highlight_style(HIGHLIGHT_STYLE)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(
            krate_list,
            frame.area().inner(Margin::new(20, 5)),
            &mut state.update_list_state,
        );

        Ok(())
    }
}

impl Selectable for Update {
    fn select(
        app: &mut crate::app::App,
        key: &crossterm::event::KeyEvent,
    ) -> Result<(), crate::errors::Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => app.view = View::StartView(Start),
            (_, KeyCode::Char('j')) => {
                if app.state.update_list_state.selected().is_none() {
                    app.state.update_list_state.select(Some(0))
                } else {
                    app.state.update_list_state.select_next();
                }
            }
            (_, KeyCode::Char('k')) => {
                if app.state.update_list_state.selected().is_none() {
                    app.state.update_list_state.select(Some(0))
                } else {
                    app.state.update_list_state.select_previous();
                }
            }
            (_, KeyCode::Enter) => todo!(),
            _ => {}
        }
        Ok(())
    }
}

