use crate::depot::DepotState;
use crate::errors::Error;
use crate::keys::key_handler;
use crate::ui::{render, views::View};
use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::DefaultTerminal;

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    running: bool,
    pub state: DepotState,
    pub view: View,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<(), Error> {
        match crossterm::event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => key_handler(self, key)?,
            _ => {}
        }
        Ok(())
    }
}
