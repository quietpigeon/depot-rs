use super::{View, start_view::Start};
use crate::depot::DepotMessage;
use crate::errors::{ChannelError, Error};
use crate::events::{AppEvent, Event};
use crate::keys::Selectable;
use crate::ui::{DEFAULT_COLOR, DEFAULT_STYLE, Drawable, HIGHLIGHT_STYLE};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
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
                    krate.krate_info.info.latest_version.clone()
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
            .highlight_symbol("* ")
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
    async fn select(app: &mut crate::app::App, key: &KeyEvent) -> Result<(), Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => {
                app.view = View::Start(Start);
            }
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
            (_, KeyCode::Enter) => {
                if let Some(ix) = app.state.update_list_state.selected() {
                    let k = &app.state.depot.get_outdated_krates()?.0[ix];
                    let kk = k.clone();
                    let tx = app.events.get_sender();
                    // Decouples the update logic to make sure this doesn't block the UI
                    tokio::spawn(async move {
                        let res = kk.update().await;
                        match res {
                            Ok(_) => {
                                tx.send(Event::App(AppEvent::Depot(DepotMessage::UpdateKrate {
                                    krate: kk.name,
                                })))
                            }
                            Err(_) => tx.send(Event::App(AppEvent::Depot(
                                DepotMessage::DepotError(ChannelError::UpdateKrate),
                            ))),
                        }
                    });
                }
            }
            _ => {}
        }

        Ok(())
    }
}
