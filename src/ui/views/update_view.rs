use super::{View, start_view::Start};
use crate::app::App;
use crate::depot::DepotMessage;
use crate::errors::{ChannelError, Error};
use crate::events::{AppEvent, Event};
use crate::keys::Selectable;
use crate::ui::{
    DEFAULT_PRIMARY_COLOR, DEFAULT_SECONDARY_COLOR, DEFAULT_STYLE, Drawable, HIGHLIGHT_STYLE,
};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph};
use throbber_widgets_tui::Throbber;

#[derive(Debug)]
pub struct Update;

impl Drawable for Update {
    fn render(
        &self,
        state: &mut crate::depot::DepotState,
        frame: &mut ratatui::Frame,
    ) -> Result<(), crate::errors::Error> {
        let outdated_krates = state.depot.get_outdated_krates()?.0.clone();
        let updating_krates = state.get_update_items();
        let mut krates: Vec<ListItem> = Vec::new();

        let throbber_style = Style::new().fg(Color::White).add_modifier(Modifier::ITALIC);
        let throbber = Throbber::default()
            .style(throbber_style)
            .to_symbol_span(&state.throbber_state);

        for krate in outdated_krates {
            let item = if updating_krates.contains(&krate.name) {
                let line = Span::raw(format!(
                    "{}  {} -> {}",
                    krate.name.clone(),
                    krate.version.clone(),
                    krate.krate_info.info.latest_version.clone()
                ))
                .style(DEFAULT_STYLE);

                let tab_spacer = Span::raw("\t");
                let label = Span::styled("updating", throbber_style);
                let line = Line::from(vec![line, tab_spacer, throbber.clone(), label]);

                ListItem::from(line)
            } else {
                ListItem::from(format!(
                    "{}  {} -> {}",
                    krate.name.clone(),
                    krate.version.clone(),
                    krate.krate_info.info.latest_version.clone()
                ))
                .fg(DEFAULT_PRIMARY_COLOR)
            };
            krates.push(item);
        }

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

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.area().inner(Margin::new(20, 5)));

        let (main_area, footer) = (layout[0], layout[1]);

        frame.render_stateful_widget(krate_list, main_area, &mut state.update_list_state);
        self.render_helpline(frame, footer)?;

        Ok(())
    }

    fn render_helpline(&self, frame: &mut Frame, area: Rect) -> Result<(), Error> {
        let line = Line::from(vec![
            Span::raw("Press "),
            Span::raw("k/j").style(Style::new().fg(DEFAULT_SECONDARY_COLOR)),
            Span::raw(" "),
            Span::raw("to move up/down"),
            Span::raw(", "),
            Span::raw("ENTER").style(Style::new().fg(DEFAULT_SECONDARY_COLOR)),
            Span::raw(" "),
            Span::raw("to update crate"),
            Span::raw(", "),
            Span::raw("q").style(Style::new().fg(DEFAULT_SECONDARY_COLOR)),
            Span::raw(" "),
            Span::raw("to go back"),
        ]);

        let footer_bar = Paragraph::new(line);
        frame.render_widget(footer_bar, area);

        Ok(())
    }
}

impl Selectable for Update {
    async fn select(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => {
                app.view = View::Start(Start);
            }
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => {
                if app.state.update_list_state.selected().is_none() {
                    app.state.update_list_state.select(Some(0))
                } else {
                    app.state.update_list_state.select_next();
                }
            }
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => {
                if app.state.update_list_state.selected().is_none() {
                    app.state.update_list_state.select(Some(0))
                } else {
                    app.state.update_list_state.select_previous();
                }
            }
            (_, KeyCode::Enter) => {
                if let Some(ix) = app.state.update_list_state.selected() {
                    let k = &app.state.depot.get_outdated_krates()?.0[ix];
                    app.state.append_to_update_queue(&k.name);

                    let kk = k.clone();
                    let tx = app.events.get_sender();
                    // Decouples the update logic to so that this doesn't block the UI
                    tokio::spawn(async move {
                        let res = kk.update().await;
                        match res {
                            Ok(_) => tx.send(Event::App(AppEvent::DepotEvent(
                                DepotMessage::UpdateKrate { krate: kk.name },
                            ))),
                            Err(_) => tx.send(Event::App(AppEvent::DepotEvent(
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
