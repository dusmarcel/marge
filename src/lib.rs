use std::path::PathBuf;
use color_eyre::eyre::Result;
use directories::ProjectDirs;
use clap::{command, arg, value_parser};

use crossterm::event::KeyCode::Char;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

mod config;
mod tui;
mod ui;

use config::Config;
use tui::{Tui, Event};
use ui::Ui;

#[derive(Clone)]
pub enum Action {
    Tick,
    Quit,
    Render,
    Domains,
    RequestResponse(String),
    None,
}

pub struct Marge {
    config_dir: Option<PathBuf>,
    config: Config,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    tui: Tui,
    ui: Ui,
}

impl Marge {
    pub fn new() -> Result<Self> {
        let mut config_dir = None;
        if let Some(project_dirs) = ProjectDirs::from("org", "keienb", "marge") {
            config_dir = Some(project_dirs.config_dir().to_path_buf());
        }
        let mut config = Config::new();
        if let Some(ref config_dir) = config_dir {
          if let Ok(lconfig) = Config::new_from_file(&config_dir) {
            config = lconfig;
          }
        }
        let should_quit = false;
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let tui = Tui::new()?;
        let ui = Ui::new();
    
        Ok(Self {
            config_dir,
            config,
            should_quit,
            action_tx,
            action_rx,
            tui,
            ui,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let matches = command!()
        .about("TUI for mailman3")
        .args([
          arg!(-u --username <USERNAME> "admin username for mailman3 REST API. See https://docs.mailman3.org/projects/mailman/en/latest/src/mailman/config/docs/config.html#admin-user")
            .required(false)
            .value_parser(value_parser!(String)),
          arg!(-p --password <PASSWORD> "admin password for mailman3 REST API. See https://docs.mailman3.org/projects/mailman/en/latest/src/mailman/config/docs/config.html#admin-pass")
            .required(false)
            .value_parser(value_parser!(String)),
          arg!(-H --host <HOST> "host to connect")
            .required(false)
            .value_parser(value_parser!(String)),
          arg!(-t --protocol <PROTOCOL> "protocol to use (i.e., http oder https")
            .required(false)
            .value_parser(value_parser!(String)),
          arg!(-P --port <PORT> "port to connect to")
            .required(false)
            .value_parser(value_parser!(i32)) 
        ])
        .try_get_matches();

        match matches {
            Err(e) => match e.kind() {
              clap::error::ErrorKind::DisplayHelp |
              clap::error::ErrorKind::DisplayVersion => {
                println!("{}", e);
                Ok(())
              },
              _ => {
                eprintln!("An error occured while parsing arguments: {}", e.to_string());
                Err(Box::new(clap::Error::new(e.kind())))?
              },     
            },

            Ok(matches) => {
                if let Some(username) = matches.get_one::<String>("username") {
                    self.config.set_username(username.to_string());
                }
                if let Some(password) = matches.get_one::<String>("password") {
                    self.config.set_password(password.to_string());
                }
                if let Some(protocol) = matches.get_one::<String>("protocol") {
                    self.config.set_protocol(protocol.to_string());
                }
                if let Some(host) = matches.get_one::<String>("host") {
                    self.config.set_host(host.to_string());
                }
                if let Some(port) = matches.get_one::<i32>("port") {
                    self.config.set_port(*port);
                }

                self.tui.enter()?;

                self.action_tx.send(Action::Domains)?;

                while !self.should_quit {
                    let e = self.tui.next().await?;
                    match e {
                        tui::Event::Quit => self.action_tx.send(Action::Quit)?,
                        tui::Event::Render => self.action_tx.send(Action::Render)?,
                        tui::Event::Key(_) => {
                            let action = self.get_action(e);
                            self.action_tx.send(action.clone())?;
                        }
                        _ => {}
                    }

                    while let Ok(action) = self.action_rx.try_recv() {
                        self.update(action.clone());
                        if let Action::Render = action {
                            self.tui.draw(|f| {
                                self.ui.render(f);
                            })?;
                        }
                    }
                }

                self.tui.exit()?;

                if let Some(config_dir) = &self.config_dir {
                    self.config.save(&config_dir);
                }

                Ok(())
            }
        }
    }

    fn get_action(&mut self, event: Event) -> Action {
        match event {
            Event::Error => Action::None,
            Event::Tick => Action::Tick,
            Event::Render => Action::Render,
            Event::Key(key) =>
                match key.code {
                    Char('q') |
                    Char('Q') => Action::Quit,
                    Char('d') |
                    Char('D') => Action::Domains,
                    _ => Action::None,
                }
            _ => Action::None       
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Domains => {
                let action_tx = self.action_tx.clone();
                let config = self.config.clone();
                tokio::spawn(async move {
                    let response = reqwest::get(format!("{}://{}:{}/3.1/domains",
                        config.protocol(),
                        config.host(),
                        config.port())).await;
                    let _ = match response {
                        Ok(body) => action_tx.send(Action::RequestResponse(body.text().await.unwrap())),
                        Err(e) => action_tx.send(Action::RequestResponse(e.to_string()))
                    };
                });
            }
            Action::RequestResponse(body) => self.ui.set_status(body),
            _ => {}
        }
    }
}