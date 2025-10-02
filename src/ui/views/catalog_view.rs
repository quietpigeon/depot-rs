use super::{View, start_view::Start};
use crate::app::App;
use crate::depot::{DepotMessage, Krate};
use crate::errors::ChannelError;
use crate::events::{AppEvent, Event};
use crate::ui::{DEFAULT_PRIMARY_COLOR, DEFAULT_SECONDARY_COLOR, DEFAULT_STYLE, HIGHLIGHT_STYLE};
use crate::{depot::DepotState, errors::Error, keys::Selectable, ui::Drawable};
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, List, ListItem, Paragraph, Wrap};
use std::rc::Rc;

#[derive(Debug)]
pub struct Catalog;

impl Drawable for Catalog {
    fn render(
        &self,
        state: &mut crate::depot::DepotState,
        frame: &mut ratatui::Frame,
    ) -> Result<(), crate::errors::Error> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.area().inner(Margin::new(5, 5)));

        let (main_area, footer) = (layout[0], layout[1]);
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Fill(1)])
            .split(main_area);

        render_left(state, frame, main_layout[0])?;

        if let Some(ix) = state.list_state.selected() {
            let krate = &state.depot.store.0[ix];
            let title = format!("| {}@{} |", krate.name, krate.version);
            let r_block = Block::bordered()
                .border_type(BorderType::Rounded)
                .style(DEFAULT_STYLE)
                .title(title);
            let inner = r_block.inner(main_layout[1]);
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

            frame.render_widget(r_block, main_layout[1]);
            render_right(krate, frame, right)?;
        }
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
            Span::raw("d").style(Style::new().fg(DEFAULT_SECONDARY_COLOR)),
            Span::raw(" "),
            Span::raw("to uninstall crate"),
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

fn render_left(state: &mut DepotState, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    render_catalog(state, frame, area)?;

    Ok(())
}

fn render_right(krate: &Krate, frame: &mut Frame, area: Rc<[Rect]>) -> Result<(), Error> {
    // NOTE: This assumes every crate must have a description.
    // This is true for crates that have been uploaded to crates.io, but it might break for local
    // crates that don't have a description yet.
    if !&krate.description().is_empty() {
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
        .map(|krate| ListItem::from(krate.name.clone()).fg(DEFAULT_PRIMARY_COLOR))
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

    let description = vec![Span::styled(krate.description(), DEFAULT_STYLE)];
    lines.push(Line::from(description));

    let spacer = vec![Span::styled("\n", DEFAULT_STYLE)];
    lines.push(Line::from(spacer));

    let tags = &krate.tags_str();
    if !tags.is_empty() {
        let tags = text_with_title(" Tags", tags)?;
        lines.push(Line::from(tags));
    }

    let license = &krate.license();
    let license = text_with_title("󰿃 License", license)?;
    lines.push(Line::from(license));

    let rv = &krate.rust_version_str();
    let rust_version = text_with_title(" Rust version", rv)?;
    lines.push(Line::from(rust_version));

    let docs = &krate.documentation();
    if !docs.is_empty() {
        let docs = text_with_title("󰈙 Documentation", docs)?;
        lines.push(Line::from(docs));
    }

    let hp = &krate.homepage();
    if !hp.is_empty() {
        let homepage = text_with_title("󰋜 Homepage", hp)?;
        lines.push(Line::from(homepage));
    }

    let repo = &krate.repository();
    if !repo.is_empty() {
        let repo = text_with_title("󰳏 Repository", repo)?;
        lines.push(Line::from(repo));
    }

    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: true }), area);

    Ok(())
}

fn text_with_title<'a>(title: &'a str, text: &'a str) -> Result<Vec<Span<'a>>, Error> {
    let lines = vec![
        Span::styled(
            format!("{title}: "),
            Style::default()
                .fg(DEFAULT_PRIMARY_COLOR)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(text, DEFAULT_STYLE),
    ];

    Ok(lines)
}

impl Selectable for Catalog {
    async fn select(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => app.view = View::Start(Start),
            // Here, we assume all of the crate info has been fetched.
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => select_next(&mut app.state)?,
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => select_previous(&mut app.state)?,
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

fn delete_selected_crate(app: &mut App) {
    if let Some(ix) = app.state.list_state.selected() {
        app.state.list_state.select(None);
        let k = &app.state.depot.store.0[ix];
        let kk = k.clone();
        let tx = app.events.get_sender();
        tokio::spawn(async move {
            let _ = match &kk.uninstall().await {
                Ok(_) => tx.send(Event::App(AppEvent::DepotEvent(
                    DepotMessage::UninstallKrate,
                ))),
                Err(_) => tx.send(Event::App(AppEvent::DepotEvent(DepotMessage::DepotError(
                    ChannelError::UninstallKrate,
                )))),
            };
        });
        // NOTE: Is it safe to assume `ix` in `app.list_state` is the same as in
        // `app.state.depot.store`?
        app.state.depot.store.0.remove(ix);
    };
}
