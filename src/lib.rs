use std::path::PathBuf;
use color_eyre::eyre::Result;
use directories::ProjectDirs;
use clap::{command, arg, value_parser};
use crossterm::event::KeyCode::{self, Char};
use request::ReqType;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tui_textarea::{Input, Key};
use reqwest::Client;
use serde_json::value::Value;

mod config;
mod tui;
mod ui;
mod request;
mod response;
mod domains;
mod lists;
mod members;
mod messages;
mod popup;
mod member_add;

use config::Config;
use tui::{Tui, Event};
use ui::{Ui, MenuItem};
use response::{ResponseType, Response};
use domains::Domains;
use lists::Lists;
use members::Members;
use messages::Messages;
use popup::Popup;
use member_add::MemberAdd;

#[derive(Clone)]
pub enum Action {
    Tick,
    Quit,
    Render,
    Domains,
    Lists,
    Members,
    Messages,
    PopupSubmit,
    Unselect,
    Up,
    Down,
    Add,
    RequestResponse(Response),
    None,
}

pub struct Marge {
    config_dir: Option<PathBuf>,
    config: Config,
    config_changed: bool,
    domains: Option<Domains>,
    lists: Option<Lists>,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    tui: Tui,
    ui: Ui,
    client: Client,
    response_t: Option<ResponseType>,
    popup: Option<Box<dyn Popup>>,
}

impl Marge {
    pub fn new() -> Result<Self> {
        let mut config_changed = false;
        let mut config_dir = None;
        if let Some(project_dirs) = ProjectDirs::from("org", "keienb", "marge") {
            config_dir = Some(project_dirs.config_dir().to_path_buf());
        }
        let mut config = Config::new();
        if let Some(ref config_dir) = config_dir {
          if let Ok(lconfig) = Config::new_from_file(&config_dir) {
            config = lconfig;
          } else {
            config_changed = true;
          }
        } else {
            config_changed = true;
        }
        let domains = None;
        let lists = None;
        let should_quit = false;
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let tui = Tui::new()?;
        let ui = Ui::new();
        let client = reqwest::Client::new();
        let response_t = None;
        let popup = None;
    
        Ok(Self {
            config_dir,
            config,
            config_changed,
            domains,
            lists,
            should_quit,
            action_tx,
            action_rx,
            tui,
            ui,
            client,
            response_t,
            popup,
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
//                      tui::Event::Quit => self.action_tx.send(Action::Quit)?,
                        tui::Event::Render => self.action_tx.send(Action::Render)?,
                        tui::Event::Key(k_event) => {
                            if let Some(_) = &self.popup {
                                match k_event.into() {
                                    Input { key: Key::Esc, .. } => self.popup = None,
                                    Input { key: Key::Enter, .. } => self.action_tx.send(Action::PopupSubmit)?,
                                    input => {
                                        self.popup.as_mut().unwrap().input(input);
                                    }
                                }
                            } else {
                                let action = self.get_action(e);
                                self.action_tx.send(action.clone())?;
                            }
                        }
                        _ => {}
                    }

                    while let Ok(action) = self.action_rx.try_recv() {
                        self.update(action.clone());
                        if let Action::Render = action {
                            self.tui.draw(|f| {
                                self.ui.render(f);
                                if let Some(_) = &self.popup {
                                    self.popup.as_mut().unwrap().render(f);
                                }
                            })?;
                        }
                    }
                }

                self.tui.exit()?;

                if self.config_changed {
                   if let Some(config_dir) = &self.config_dir {
                        self.config.set_domain(None);
                        self.config.set_list(None);
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
                    Char('l') |
                    Char('L') => Action::Lists,
                    Char('m') |
                    Char('M') => Action::Members,
                    Char('s') |
                    Char('S') => Action::Messages,
                    Char('j') |
                    Char('J') |
                    KeyCode::Down => Action::Down,
                    Char('k') |
                    Char('K') |
                    KeyCode::Up => Action::Up,
                    Char('u') |
                    Char('U') |
                    KeyCode::Backspace => Action::Unselect,
                    Char('a') |
                    Char('A') => Action::Add,
                    //Enter => Action::Select,
                    _ => Action::None,
                }
            _ => Action::None       
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Domains => {
                self.ui.set_active_menu_item(MenuItem::Domains);
                let action_tx = self.action_tx.clone();
                let mut client = self.client.clone();
                let config = self.config.clone();
                tokio::spawn(async move {
                    let resp = request::request(&mut client, ReqType::Domains, &config).await;
                    let response = Response::new(resp, ResponseType::Domains).await;
                    let _ = action_tx.send(Action::RequestResponse(response));
                });
            }
            Action::Lists => {
                self.ui.set_active_menu_item(MenuItem::Lists);
                let action_tx = self.action_tx.clone();
                let mut client = self.client.clone();
                let config = self.config.clone();
                tokio::spawn(async move {
                    let resp = request::request(&mut client, ReqType::Lists, &config).await;
                    let response = Response::new(resp, ResponseType::Lists).await;
                    let _ = action_tx.send(Action::RequestResponse(response));
                });    
            }
            Action::Members => {
                self.ui.set_active_menu_item(MenuItem::Members);
                let action_tx = self.action_tx.clone();
                let mut client = self.client.clone();
                let config = self.config.clone();
                tokio::spawn(async move {
                    let resp = request::request(&mut client, ReqType::Members, &config).await;
                    let response = Response::new(resp, ResponseType::Members).await;
                    let _ = action_tx.send(Action::RequestResponse(response));
                });
            }
            Action::Messages => {
                if let Some(_) = &self.config.list() {
                    self.ui.set_active_menu_item(MenuItem::Messages);
                    let action_tx = self.action_tx.clone();
                    let mut client = self.client.clone();
                    let config = self.config.clone();
                    tokio::spawn(async move {
                        let resp = request::request(&mut client, ReqType::Messages, &config).await;
                        let response = Response::new(resp, ResponseType::Messages).await;
                        let _ = action_tx.send(Action::RequestResponse(response));
                    });
                } else {
                    self.ui.set_status("Can't fetch messages: No list selected!".to_string());
                }
            }
            Action::PopupSubmit => {
                let action_tx = self.action_tx.clone();
                let mut client = self.client.clone();
                let config = self.config.clone();
                let params = self.popup.as_ref().unwrap().submit();
                let response_t = self.response_t.clone();
                tokio::spawn(async move {
                    let resp = request::request(&mut client, ReqType::Popup(params), &config).await;
                    let _response = Response::new(resp, ResponseType::Messages).await;
                    if let Some(response_t) = response_t {
                        let _ = match response_t {
                            ResponseType::Domains => action_tx.send(Action::Domains),
                            ResponseType::Lists => action_tx.send(Action::Lists),
                            ResponseType::Members => action_tx.send(Action::Members),
                            ResponseType::Messages => action_tx.send(Action::Messages),
                        };
                    }
                });
                self.popup = None;                
            }
            Action::Down => {
                self.ui.down();
                if let Some(i) = self.ui.selected() {
                    if let Some(response_type) = &self.response_t {
                        match response_type {
                            ResponseType::Domains => if let Some(domains) = &self.domains {
                                self.config.set_domain(Some(domains.entries()[i].clone()));
                                self.ui.set_sel_domain(Some(domains.entries()[i].mail_host()));
                            }
                            ResponseType::Lists => if let Some(lists) = &self.lists {
                                self.config.set_list(Some(lists.entries()[i].clone()));
                                self.ui.set_sel_list(Some(lists.entries()[i].display_name()));
                            }                   
                            _ => {}
                        }
                    }
                }
            }
            Action::Up => {
                self.ui.up();
                if let Some(i) = self.ui.selected() {
                    if let Some(response_type) = &self.response_t {
                        match response_type {
                            ResponseType::Domains => if let Some(domains) = &self.domains {
                                self.config.set_domain(Some(domains.entries()[i].clone()));
                                self.ui.set_sel_domain(Some(domains.entries()[i].mail_host()));
                            }
                            ResponseType::Lists => if let Some(lists) = &self.lists {
                                self.config.set_list(Some(lists.entries()[i].clone()));
                                self.ui.set_sel_list(Some(lists.entries()[i].display_name()));
                            }                            
                            _ => {}
                        }
                    }
                }
            }
            Action::Unselect => {
                self.ui.select(None);
                if let Some(response_type) = &self.response_t {
                    match response_type {
                        ResponseType::Domains => {
                            self.config.set_domain(None);
                            self.ui.select(None);
                            self.ui.set_sel_domain(None);
                        }
                        ResponseType::Lists => {
                            self.config.set_list(None);
                            self.ui.select(None);
                            self.ui.set_sel_list(None);
                        }
                        _ => {}
                    }
                }
            }
            Action::Add => {
                if let Some(response_t) = &self.response_t {
                    if *response_t == ResponseType::Members {
                        if let Some(_list) = self.config.list() {
                            self.popup = Some(Box::new(MemberAdd::new(self.config.clone())));
                        }
                        else {
                            self.ui.set_status("You must select a list before I can add members.".to_string());
                        }
                    } else {
                        self.ui.set_status("Sorry, don't know yet how to add new items here...".to_string());
                    }
                } else {
                    self.ui.set_status("Sorry, nothing to add here".to_string());
                }
            }
            Action::RequestResponse(response) => {
                self.response_t = Some(response.response_type());
                match response.response_type() {
                    ResponseType::Domains => {
                        let domains: Result<Domains, serde_json::Error> = serde_json::from_str(&response.text());
                        match domains {
                            Ok(domains) => {
                                self.domains = Some(domains.clone());
                                self.ui.set_list_vec(domains.clone().list_vec());
                                self.config.set_domain(Some(domains.entries()[0].clone()));
                                self.ui.set_sel_domain(Some(domains.entries()[0].mail_host()));
                            }
                            Err(e) => {
                                self.domains = None;
                                self.ui.set_list_vec(vec![format!("Error: {}", e.to_string())]);
                            }
                        }
                    }
                    ResponseType::Lists => {
                        let lists: Result<Lists, serde_json::Error> = serde_json::from_str(&response.text());
                        match lists {
                            Ok(lists) => {
                                self.lists = Some(lists.clone());
                                self.ui.set_list_vec(lists.clone().list_vec());
                                self.config.set_list(Some(lists.entries()[0].clone()));
                                self.ui.set_sel_list(Some(lists.entries()[0].display_name()));
                            }
                            Err(e) => {
                                self.lists = None;
                                self.ui.set_list_vec(vec![format!("Error: {}", e.to_string())])
                            }
                        }                        
                    },
                    ResponseType::Members => {
                        let members: Result<Members, serde_json::Error> = serde_json::from_str(&response.text());
                        match members {
                            Ok(members) => {
                                self.ui.set_list_vec(members.clone().list_vec());
                            }
                            Err(e) => {
                                if let Ok(value) = serde_json::from_str::<Value>(&response.text()) {
                                    self.ui.set_list_vec(vec![format!("Error: {}", e.to_string()), format!("Original response value: {:#?}", value)]);
                                } else {
                                    self.ui.set_list_vec(vec![format!("Error: {}", e.to_string()), format!("Original response text: {}", response.text())]);
                                }
                            }
                        }                          
                    },
                    ResponseType::Messages => {
                        let messages: Result<Messages, serde_json::Error> = serde_json::from_str(&response.text());
                        match messages {
                            Ok(messages) => {
                                self.ui.set_list_vec(messages.clone().list_vec());
                            }
                            Err(e) => {
                                if let Ok(value) = serde_json::from_str::<Value>(&response.text()) {
                                    self.ui.set_list_vec(vec![format!("Error: {}", e.to_string()), format!("Original response value: {:#?}", value)]);
                                } else {
                                    self.ui.set_list_vec(vec![format!("Error: {}", e.to_string()), format!("Original response text: {}", response.text())]);
                                }
                            }
                        }                        
                    },
                }
                self.ui.set_status(response.status());
            }
            _ => {}
        }
    }
}