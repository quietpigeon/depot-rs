use super::{View, start_view::Start};
use crate::depot::Krate;
use crate::ui::{DEFAULT_COLOR, DEFAULT_STYLE, HIGHLIGHT_STYLE};
use crate::{depot::DepotState, errors::Error, keys::Selectable, ui::Drawable};
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, List, ListItem, Paragraph, Wrap};
use std::rc::Rc;

#[derive(Debug)]
pub struct Catalog;

impl Drawable for Catalog {
    fn render(
        state: &mut crate::depot::DepotState,
        frame: &mut ratatui::Frame,
    ) -> Result<(), crate::errors::Error> {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Fill(1)])
            .split(frame.area().inner(Margin::new(5, 5)));
        let left = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Fill(1)])
            .split(layout[0]);
        let right = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Fill(4),
            ])
            .split(layout[1]);

        render_left(state, frame, left)?;
        render_right(state, frame, right)?;

        Ok(())
    }
}
fn render_left(state: &mut DepotState, frame: &mut Frame, area: Rc<[Rect]>) -> Result<(), Error> {
    render_catalog(state, frame, area[0])?;
    if let Some(ix) = state.list_state.selected() {
        let krate = &state.depot.store.0[ix];
        if !krate.info.description.is_empty() {
            render_tag_list(krate, frame, area[1])?;
        }
    }

    Ok(())
}

fn render_right(state: &mut DepotState, frame: &mut Frame, area: Rc<[Rect]>) -> Result<(), Error> {
    let middle = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Fill(1),
        ])
        .split(area[1]);

    if let Some(ix) = state.list_state.selected() {
        let krate = &state.depot.store.0[ix];
        if !krate.info.description.is_empty() {
            render_description(krate, frame, area[0])?;
            render_version(krate, frame, middle[0])?;
            render_license(krate, frame, middle[1])?;
            render_rust_version(krate, frame, middle[2])?;
            render_documentation_url(krate, frame, area[2])?;
            render_homepage(krate, frame, area[3])?;
            render_repository_url(krate, frame, area[4])?;
        }
    }

    Ok(())
}

fn render_catalog(state: &mut DepotState, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let krates: Vec<ListItem> = state
        .depot
        .store
        .0
        .iter()
        .map(|krate| ListItem::from(krate.name.clone()).fg(DEFAULT_COLOR))
        .collect();
    let krate_list = List::new(krates)
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Installed crates")
                .style(DEFAULT_STYLE),
        )
        .highlight_symbol("* ")
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
    frame.render_stateful_widget(krate_list, area, &mut state.list_state);

    Ok(())
}

fn render_tag_list(krate: &Krate, frame: &mut ratatui::Frame, area: Rect) -> Result<(), Error> {
    let tags: Vec<ListItem> = krate
        .info
        .tags
        .0
        .iter()
        .map(|t| ListItem::from(format!("#{t}")).fg(DEFAULT_COLOR))
        .collect();
    let tag_list = List::new(tags).block(
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Tags")
            .style(DEFAULT_STYLE),
    );

    frame.render_widget(tag_list, area);

    Ok(())
}

fn render_description(krate: &Krate, frame: &mut ratatui::Frame, area: Rect) -> Result<(), Error> {
    let description = &krate.info.description;
    render_text_with_title("Description", description, frame, area)?;

    Ok(())
}

fn render_version(krate: &Krate, frame: &mut ratatui::Frame, area: Rect) -> Result<(), Error> {
    let version = &krate.version;
    render_text_with_title("Version", version.to_string().as_str(), frame, area)?;

    Ok(())
}

fn render_license(krate: &Krate, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let license = &krate.info.license;
    render_text_with_title("License", license, frame, area)?;

    Ok(())
}

fn render_rust_version(krate: &Krate, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    if let Some(license) = &krate.info.rust_version {
        render_text_with_title("Rust version", license.to_string().as_str(), frame, area)?;
    } else {
        render_text_with_title("Rust version", "unknown", frame, area)?;
    }

    Ok(())
}

fn render_documentation_url(krate: &Krate, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let url = &krate.info.documentation;
    render_text_with_title("Documentation", url, frame, area)
}

fn render_homepage(krate: &Krate, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let url = &krate.info.homepage;
    render_text_with_title("Homepage", url, frame, area)
}

fn render_repository_url(krate: &Krate, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let url = &krate.info.repository;
    render_text_with_title("Repository", url, frame, area)
}

fn render_text_with_title(
    title: &str,
    text: &str,
    frame: &mut Frame,
    area: Rect,
) -> Result<(), Error> {
    frame.render_widget(
        Paragraph::new(text).wrap(Wrap { trim: true }).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title)
                .style(DEFAULT_STYLE),
        ),
        area,
    );
    Ok(())
}

impl Selectable for Catalog {
    async fn select(
        app: &mut crate::app::App,
        key: &crossterm::event::KeyEvent,
    ) -> Result<(), crate::errors::Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => app.view = View::Start(Start),
            // Here, we assume all of the crate info has been fetched.
            (_, KeyCode::Char('j')) => select_next(&mut app.state)?,
            (_, KeyCode::Char('k')) => select_previous(&mut app.state)?,
            _ => {}
        }
        Ok(())
    }
}

fn select_next(state: &mut DepotState) -> Result<(), Error> {
    if state.list_state.selected().is_none() {
        state.list_state.select_first();
        return Ok(());
    }
    // Prevents selecting an index out of bounds. This is most likely a bug on ratatui's
    // side.
    if state.list_state.selected().unwrap() + 1 == state.depot.crate_count as usize {
        Ok(())
    } else {
        state.list_state.select_next();
        Ok(())
    }
}

fn select_previous(state: &mut DepotState) -> Result<(), Error> {
    state.list_state.select_previous();
    Ok(())
}
