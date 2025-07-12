use crate::depot::DepotState;
use crate::errors::Error;
use crate::keys::key_handler;
use crate::ui::{render, views::View};
use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::sync::mpsc::channel;
use std::time::Duration;

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    running: bool,
    pub state: DepotState,
    pub view: View,
    pub tx: std::sync::mpsc::Sender<AppMessage>,
    pub rx: std::sync::mpsc::Receiver<AppMessage>,
    has_initialized: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let (tx, rx) = channel::<AppMessage>();
        let has_initialized = false;

        Self {
            running: true,
            state: DepotState::default(),
            view: View::default(),
            tx,
            rx,
            has_initialized,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Error> {
        self.running = true;
        while self.running {
            terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
            self.handle_crossterm_events().await?;
            self.handle_init().await?;

            // Non-blocking receiver.
            if let Ok(message) = self.rx.try_recv() {
                handle_app_message(&mut self.state, message)?;
                // Redraw to update components.
                terminal.draw(|f| {
                    render(&mut self.view, &mut self.state, f).expect("failed to render")
                })?;
            }
        }

        Ok(())
    }

    /// Run only once when the app initializes.
    async fn handle_init(&mut self) -> Result<(), Error> {
        if !self.has_initialized {
            self.state.sync().await?;
        }
        self.has_initialized = true;

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

fn handle_app_message(state: &mut DepotState, message: AppMessage) -> Result<(), Error> {
    match message {
        AppMessage::UpdateCrateSuccess { krate } => state.sync_krate(&krate)?,
        AppMessage::UninstallCrateSuccess => {}
        AppMessage::UninstallCrateFailed { krate } | AppMessage::UpdateCrateFailed { krate } => {
            return Err(Error::Unexpected(krate));
        }
    }

    Ok(())
}

pub enum AppMessage {
    UpdateCrateSuccess { krate: String },
    UpdateCrateFailed { krate: String },
    UninstallCrateSuccess,
    UninstallCrateFailed { krate: String },
}
