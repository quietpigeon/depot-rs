use crate::depot::{DepotMessage, DepotState, NamedKrateInfo};
use crate::errors::Error;
use crate::events::{AppEvent, Event, EventHandler};
use crate::keys::key_handler;
use crate::ui::{render, views::View};
use ratatui::DefaultTerminal;

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    running: bool,
    events: EventHandler,
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
            events: EventHandler::new(),
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
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|f| render(&mut self.view, &mut self.state, f).unwrap())?;
            self.handle_init().unwrap();
            match self.events.next().await? {
                Event::Tick => self.on_tick(),
                Event::Crossterm(event) => match event {
                    ratatui::crossterm::event::Event::Key(key_event) => {
                        key_handler(&mut self, key_event).await?
                    }
                    _ => {}
                },
                Event::App(event) => match event {
                    AppEvent::Depot(msg) => msg.handle(&mut self.state).unwrap(),
                },
            }
        }
        Ok(())
    }

    /// Run only once when the app initializes.
    fn handle_init(&mut self) -> Result<(), Error> {
        if !self.has_initialized {
            let names: Vec<String> = self
                .state
                .depot
                .store
                .0
                .iter()
                .map(|k| k.name.clone())
                .collect();
            let sender = self.events.get_sender();

            tokio::spawn(async move {
                let resp: Result<Vec<NamedKrateInfo>, Error> =
                    names.iter().map(|n| NamedKrateInfo::get(n)).collect();

                match resp {
                    Ok(r) => {
                        sender.send(Event::App(AppEvent::Depot(DepotMessage::FetchKrateInfo(r))))
                    }
                    Err(_) => sender.send(Event::App(AppEvent::Depot(DepotMessage::DepotError(
                        crate::errors::ChannelError::KrateInfo,
                    )))),
                }
            });
        }
        self.has_initialized = true;

        Ok(())
    }

    fn on_tick(&mut self) {
        self.state.throbber_state.calc_next();
    }
}
