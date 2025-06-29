use super::catalog_view::Catalog;
use super::update_view::Update;
use super::{Drawable, View, banner, center};
use crate::ui::DEFAULT_STYLE;
use crate::ui::components::{progress_bar, select_menu};
use crate::{app::App, depot::DepotState, errors::Error, keys::Selectable};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use std::ops::Not;

#[derive(Debug)]
pub struct Start;

impl Drawable for Start {
    fn render(state: &mut DepotState, frame: &mut Frame) -> Result<(), Error> {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(5),
                Constraint::Percentage(25),
            ])
            .split(frame.area());

        let banner = Text::raw(banner::BANNER);
        let banner_area = center(
            layout[0],
            Constraint::Length(banner.width() as u16),
            Constraint::Length(banner.height() as u16),
        );

        frame.render_widget(
            Paragraph::new(banner).style(DEFAULT_STYLE).centered(),
            banner_area,
        );

        if state.synced.not() {
            // TODO: Use progress bar instead of static text.
            frame.render_widget(progress_bar::new()?, layout[2]);
        } else {
            let outdated_krate_count = state.depot.outdated_krate_count()?;
            let outdated_crate_str = if outdated_krate_count != 0 {
                format!("{} crates are outdated.", &outdated_krate_count)
            } else {
                "All crates are up-to-date!".to_string()
            };
            frame.render_widget(
                Paragraph::new(format!(
                    "You have {} crates installed.\n\n{outdated_crate_str}",
                    state.depot.crate_count()
                ))
                .style(DEFAULT_STYLE)
                .centered(),
                layout[1],
            );

            frame.render_widget(select_menu::new()?, layout[3]);
        }

        Ok(())
    }
}

impl Selectable for Start {
    async fn select(app: &mut App, key: &crossterm::event::KeyEvent) -> Result<(), Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Char('c')) => {
                if app.state.synced {
                    app.view = View::Catalog(Catalog)
                }
            }
            (_, KeyCode::Char('u')) => app.view = View::Update(Update),
            _ => {}
        }

        Ok(())
    }
}
