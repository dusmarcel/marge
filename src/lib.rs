use std::path::PathBuf;
use color_eyre::eyre::Result;
use directories::ProjectDirs;
use clap::{command, arg, value_parser};
use crossterm::event::KeyCode::{Char, Enter};
use request::Page;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use reqwest::Client;

mod config;
mod tui;
mod ui;
mod request;
mod response;
mod domains;

use config::Config;
use tui::{Tui, Event};
use ui::Ui;
use response::{ResponseType, Response};
use domains::Domains;

#[derive(Clone)]
pub enum Action {
    Tick,
    Quit,
    Render,
    Domains,
    Select,
    RequestResponse(Response),
    None,
}

pub struct Marge {
    config_dir: Option<PathBuf>,
    config: Config,
    config_changed: bool,
    domains: Option<Domains>,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    tui: Tui,
    ui: Ui,
    client: Client,
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
        let config_changed = false;
        let domains = None;
        let should_quit = false;
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let tui = Tui::new()?;
        let ui = Ui::new();
        let client = reqwest::Client::new();
    
        Ok(Self {
            config_dir,
            config,
            config_changed,
            domains,
            should_quit,
            action_tx,
            action_rx,
            tui,
            ui,
            client,
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
                    self.config_changed = true;
                }
                if let Some(password) = matches.get_one::<String>("password") {
                    self.config.set_password(password.to_string());
                    self.config_changed = true;
                }
                if let Some(protocol) = matches.get_one::<String>("protocol") {
                    self.config.set_protocol(protocol.to_string());
                    self.config_changed = true;
                }
                if let Some(host) = matches.get_one::<String>("host") {
                    self.config.set_host(host.to_string());
                    self.config_changed = true;
                }
                if let Some(port) = matches.get_one::<i32>("port") {
                    self.config.set_port(*port);
                    self.config_changed = true;
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

                if self.config_changed {
                   if let Some(config_dir) = &self.config_dir {
                        self.config.save(&config_dir);
                    }
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
                    Enter => Action::Select,
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
                let mut client = self.client.clone();
                let config = self.config.clone();
                tokio::spawn(async move {
                    let resp = request::request(&mut client, Page::Domains, &config).await;
                    let response = Response::new(resp, ResponseType::Domains).await;
                    let _ = action_tx.send(Action::RequestResponse(response));
                });
            }
            Action::Select => {
                if let Some(i) = self.ui.selected() {
                    // This is REALLY ugly, I will work on that
                    eprintln!("{:#?}", self.domains.clone().unwrap().entries()[i]);
                }
            }
            Action::RequestResponse(response) => {
                match  response.response_type() {
                    ResponseType::Domains => {
                        let domains: Result<Domains, serde_json::Error> = serde_json::from_str(&response.text());
                        match domains {
                            Ok(domains) => {
                                self.domains = Some(domains.clone());
                                self.ui.set_list_vec(domains.clone().list_vec());
                            }
                            Err(e) => self.ui.set_list_vec(vec![format!("Error: {}", e.to_string())])
                        }
                        self.ui.set_status(response.status());
                    }
                    _ => {},
                }
            }
            _ => {}
        }
    }
}