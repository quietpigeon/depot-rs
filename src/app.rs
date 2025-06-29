use crate::depot::DepotState;
use crate::errors::Error;
use crate::keys::key_handler;
use crate::ui::{render, views::View};
use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::ops::Not;
use std::sync::mpsc::{self, channel};
use std::time::Duration;

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    running: bool,
    pub state: DepotState,
    pub view: View,
    pub tx: mpsc::Sender<AppMessage>,
    pub rx: mpsc::Receiver<AppMessage>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let (tx, rx) = channel::<AppMessage>();

        Self {
            running: true,
            state: DepotState::default(),
            view: View::default(),
            tx,
            rx,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
            self.handle_crossterm_events().await?;

            if self.state.synced.not() {
                self.state.sync()?;
            }

            if let Ok(message) = self.rx.try_recv() {
                match message {
                    AppMessage::UpdateCrateSuccess { krate } => {
                        self.state.sync_krate(&krate)?;
                        terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
                    }
                    AppMessage::UninstallCrateSuccess => {
                        terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
                    }
                    AppMessage::UpdateCrateFailed { krate } => println!("failed updating {krate}"),
                    AppMessage::UninstallCrateFailed { krate } => {
                        println!("failed uninstalling {krate}")
                    }
                }
            }
        }

        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    async fn handle_crossterm_events(&mut self) -> Result<(), Error> {
        if crossterm::event::poll(Duration::from_millis(50))? {
            match crossterm::event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    key_handler(self, key).await?
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub enum AppMessage {
    UpdateCrateSuccess { krate: String },
    UpdateCrateFailed { krate: String },
    UninstallCrateSuccess,
    UninstallCrateFailed { krate: String },
}
