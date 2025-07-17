use crate::depot::DepotState;
use crate::errors::Error;
use crate::keys::key_handler;
use crate::ui::{render, views::View};
use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::poll;

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    running: bool,
    pub state: DepotState,
    pub view: View,
    has_initialized: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let has_initialized = false;

        Self {
            running: true,
            state: DepotState::default(),
            view: View::default(),
            has_initialized,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Run the application's main loop.
    pub async fn run(
        mut self,
        mut terminal: DefaultTerminal,
        tick_rate: std::time::Duration,
    ) -> Result<(), Error> {
        self.running = true;
        while self.running {
            terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
            self.on_tick();
            self.handle_init()?;

            if poll(tick_rate)? {
                self.handle_crossterm_events().await?;
            }

            if let Ok(msg) = self.state.rx.try_recv() {
                msg.handle(&mut self.state)?;
                terminal.draw(|f| {
                    render(&mut self.view, &mut self.state, f).expect("failed to render")
                })?;
            }
        }

        Ok(())
    }

    /// Run only once when the app initializes.
    fn handle_init(&mut self) -> Result<(), Error> {
        if !self.has_initialized {
            self.state.load_info()?;
        }
        self.has_initialized = true;

        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    async fn handle_crossterm_events(&mut self) -> Result<(), Error> {
        match crossterm::event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => key_handler(self, key).await?,
            _ => {}
        }
        Ok(())
    }

    fn on_tick(&mut self) {
        self.state.throbber_state.calc_next();
    }
}
