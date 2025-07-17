use super::{View, start_view::Start};
use crate::depot::{DepotMessage, Krate};
use crate::ui::{DEFAULT_COLOR, DEFAULT_STYLE, HIGHLIGHT_STYLE};
use crate::{depot::DepotState, errors::Error, keys::Selectable, ui::Drawable};
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
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

        render_left(state, frame, layout[0])?;

        if let Some(ix) = state.list_state.selected() {
            let krate = &state.depot.store.0[ix];
            let title = format!("| {}@{} |", krate.name, krate.version);
            let r_block = Block::bordered()
                .border_type(BorderType::Rounded)
                .style(DEFAULT_STYLE)
                .title(title);
            let inner = r_block.inner(layout[1]);
            let right = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(vec![
                    // Decription.
                    Constraint::Percentage(20),
                    // License
                    Constraint::Percentage(10),
                    //Rust version.
                    Constraint::Percentage(10),
                    // Documentation.
                    Constraint::Percentage(10),
                    // Homepage.
                    Constraint::Percentage(10),
                    //Repository.
                    Constraint::Percentage(10),
                ])
                .split(inner);

            frame.render_widget(r_block, layout[1]);
            render_right(krate, frame, right)?;
        }

        Ok(())
    }
}
fn render_left(state: &mut DepotState, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    render_catalog(state, frame, area)?;

    Ok(())
}

fn render_right(krate: &Krate, frame: &mut Frame, area: Rc<[Rect]>) -> Result<(), Error> {
    // NOTE: This assumes every crate must have a description.
    // This is true for crates that have been uploaded to crates.io, but it might break for local
    // crates that don't have a description yet.
    if !krate.krate_info.info.description.is_empty() {
        render_krate_summary(krate, frame, area[0])?;
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

fn render_krate_summary(
    krate: &Krate,
    frame: &mut ratatui::Frame,
    area: Rect,
) -> Result<(), Error> {
    let mut lines = vec![];
    let d = &krate.krate_info.info.description;
    let t = &krate.krate_info.info.tags.to_string();
    let description = vec![Span::styled(d, DEFAULT_STYLE)];
    let spacer = vec![Span::styled("\n", DEFAULT_STYLE)];
    let tags = text_with_title(" Tags", t)?;
    let license = text_with_title("󰿃 License", &krate.krate_info.info.license)?;
    let rv = match &krate.krate_info.info.rust_version {
        Some(v) => v.to_string(),
        None => "unknown".to_string(),
    };
    let rust_version = text_with_title(" Rust version", &rv)?;
    let docs = text_with_title("󰈙 Documentation", &krate.krate_info.info.documentation)?;
    let hp = text_with_title("󰋜 Homepage", &krate.krate_info.info.homepage)?;
    let repo = text_with_title("󰳏 Repository", &krate.krate_info.info.repository)?;

    lines.push(Line::from(description));
    lines.push(Line::from(spacer));
    if !&krate.krate_info.info.tags.0.is_empty() {
        lines.push(Line::from(tags));
    }
    lines.push(Line::from(license));
    lines.push(Line::from(rust_version));
    lines.push(Line::from(docs));
    lines.push(Line::from(hp));
    lines.push(Line::from(repo));

    let text = Text::from(lines);

    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: true }), area);

    Ok(())
}

fn text_with_title<'a>(title: &'a str, text: &'a str) -> Result<Vec<Span<'a>>, Error> {
    let lines = vec![
        Span::styled(
            format!("{title}: "),
            Style::default()
                .fg(DEFAULT_COLOR)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(text, DEFAULT_STYLE),
    ];

    Ok(lines)
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
            (_, KeyCode::Char('d')) => delete_selected_crate(app),
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
    if state.list_state.selected().unwrap() + 1 == state.depot.crate_count() as usize {
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

fn delete_selected_crate(app: &mut crate::app::App) {
    if let Some(ix) = app.state.list_state.selected() {
        app.state.list_state.select(None);
        let k = &app.state.depot.store.0[ix];
        let kk = k.clone();
        let tx = app.state.tx.clone();
        tokio::spawn(async move {
            let _ = match &kk.uninstall().await {
                Ok(_) => tx.send(DepotMessage::UninstallKrate),
                Err(_) => tx.send(DepotMessage::DepotError(
                    crate::errors::ChannelError::UninstallKrate,
                )),
            };
        });
        // NOTE: Is it safe to assume `ix` in `app.list_state` is the same as in
        // `app.state.depot.store`?
        app.state.depot.store.0.remove(ix);
    };
}
