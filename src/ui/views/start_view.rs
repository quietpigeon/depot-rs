use super::catalog_view::Catalog;
use super::{Drawable, View, banner, center};
use crate::ui::DEFAULT_STYLE;
use crate::ui::components::progress_bar;
use crate::{app::App, depot::DepotState, errors::Error, keys::Selectable};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::text::Text;
use ratatui::widgets::Paragraph;

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
        frame.render_widget(
            Paragraph::new(format!(
                "You have {} crates installed.",
                state.depot.crate_count
            ))
            .style(DEFAULT_STYLE)
            .centered(),
            layout[1],
        );

        frame.render_widget(progress_bar::new(state)?, layout[2]);

        Ok(())
    }
}

impl Selectable for Start {
    fn select(app: &mut App, key: &crossterm::event::KeyEvent) -> Result<(), Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Char('c')) => {
                if app.state.synced {
                    app.view = View::CatalogView(Catalog)
                }
            }
            _ => {}
        }

        Ok(())
    }
}
