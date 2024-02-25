use color_eyre::eyre::Result;

use crossterm::event::KeyCode::Char;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

mod tui;

use tui::{Tui, Event};

#[derive(Clone)]
pub enum Action {
    Quit,
    None,
}

pub struct Marge {
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    tui: Tui,
}

impl Marge {
    pub fn new() -> Result<Self> {
        let should_quit = false;
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let tui = Tui::new()?;

        Ok(Self {
            should_quit,
            action_tx,
            action_rx,
            tui,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.tui.enter()?;

        while !self.should_quit {
            let e = self.tui.next().await?;
            match e {
                tui::Event::Quit => self.action_tx.send(Action::Quit)?,
                tui::Event::Key(_) => {
                    let action = self.get_action(e);
                    self.action_tx.send(action.clone())?;
                }
                _ => {}
            }

            while let Ok(action) = self.action_rx.try_recv() {
                self.update(action)
            }
        }

        self.tui.exit()?;

        Ok(())
    }

    fn get_action(&mut self, event: Event) -> Action {
        match event {
            Event::Error => Action::None,
            Event::Key(key) =>
                match key.code {
                    Char('q') |
                    Char('Q') => Action::Quit,
                    _ => Action::None,
                }
            _ => Action::None       
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            _ => {}
        }
    }
}