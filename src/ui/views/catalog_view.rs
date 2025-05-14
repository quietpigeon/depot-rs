use super::{View, start_view::Start};
use crate::ui::{DEFAULT_STYLE, HIGHLIGHT_STYLE};
use crate::{depot::DepotState, errors::Error, keys::Selectable, ui::Drawable};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout, Rect};
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

        if let Some(ix) = state.list_state.selected() {
            let _ = render_tag_list(state, frame, ix, left[1]);
        }

        frame.render_stateful_widget(krate_list, left[0], &mut state.list_state);

        Ok(())
    }
}

fn render_tag_list(
    state: &mut DepotState,
    frame: &mut ratatui::Frame,
    ix: usize,
    area: Rect,
) -> Result<(), Error> {
    {
        let krate = &state.depot.store.0[ix];
        let tags: Vec<ListItem> = krate
            .info
            .tags
            .0
            .iter()
            .map(|t| ListItem::from(format!("#{t}")).fg(Color::Yellow))
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
            (_, KeyCode::Enter) => select_crate(&mut app.state)?,
            _ => {}
        }
        Ok(())
    }
}

fn select_next(state: &mut DepotState) -> Result<(), Error> {
    if state.list_state.selected() == None {
        return Ok(state.list_state.select_first());
    }
    // Prevents selecting an index out of bounds. This is most likely a bug on ratatui's
    // side.
    if state.list_state.selected().unwrap() + 1 == state.depot.crate_count as usize {
        return Ok(());
    } else {
        Ok(state.list_state.select_next())
    }
}

fn select_previous(state: &mut DepotState) -> Result<(), Error> {
    Ok(state.list_state.select_previous())
}

fn select_crate(state: &mut DepotState) -> Result<(), Error> {
    // NOTE: This is a safe unwrap.
    let i = state.list_state.selected().unwrap();
    let krate = &state.depot.store.0[i];

    // NOTE: Might sideload this somewhere to reduce loading time.
    state.sync_krate(krate.name.clone().as_str())?;

    Ok(())
}
