use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};
use reqwest::{Method, Url};
use tui_textarea::{Input, Key};

use crate::{config::Config, popup::{Popup, PopupReqParam, PopupStatus}};

#[derive(Clone)]
enum ModAction {
    Discard,
    Reject,
    Defer,
    Accept
}

#[derive(Clone)]
pub struct MessageMod<'a> {
    config: Config,
    paragraph: Paragraph<'a>,
    action: ModAction,
}

impl<'a> MessageMod<'a> {
    pub fn new(config: Config) -> Self {
        let text = vec![
            Line::from(config.message().unwrap().description()),
            Line::from(""),
            Line::from("Type 'd' to discard the message"),
            Line::from("Type 'r' to reject the message"),
            Line::from("Type Esc to defer"),
            Line::from("Type 'a' or Enter to accept the message")
        ];
        let paragraph = Paragraph::new(text)
            .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Message Moderation ".to_string())
            .style(Style::default().fg(Color::Blue)),
        );
        let action = ModAction::Defer;

        Self {
            config,
            paragraph,
            action,
        }
    }
}

impl Popup for MessageMod<'_> {
    fn render(&mut self, frame: &mut Frame) {
        let area = Rect {
            width: 80,
            height: 8,
            x: 42,
            y: 20,
        };

        frame.render_widget(self.paragraph.clone(), area);
    }

    fn input(&mut self, input: Input) -> PopupStatus {
        let mut status = PopupStatus::Continue;
        match input {
            Input { key: Key::Char('d'), .. } |
            Input { key: Key::Char('D'), .. } => {
                self.action = ModAction::Discard;
                status = PopupStatus::Submit;
            },
            Input { key: Key::Char('r'), .. } |
            Input { key: Key::Char('R'), .. } => {
                self.action = ModAction::Reject;
                status = PopupStatus::Submit
            },
            Input { key: Key::Esc, .. } => status = PopupStatus::Cancel,
            Input { key: Key::Enter, .. } |
            Input { key: Key::Char('a'), .. } |
            Input { key: Key::Char('A'), .. } => {
                self.action = ModAction::Accept;
                status = PopupStatus::Submit
            },
            _input => {}
        }

        status
    }

    fn submit(&self) -> PopupReqParam {
        let url = Url::parse(&format!("{}://{}:{}/3.1/lists/{}/held/{}",
            self.config.protocol(),
            self.config.host(),
            self.config.port(),
            self.config.list().unwrap().fqdn_listname(),
            self.config.message().unwrap().request_id())).unwrap();
        let mut map = HashMap::new();
        match self.action {
            ModAction::Discard => map.insert("action".to_string(), "discard".to_string()),
            ModAction::Reject => map.insert("action".to_string(), "reject".to_string()),
            ModAction::Defer => map.insert("action".to_string(), "defer".to_string()),
            ModAction::Accept => map.insert("action".to_string(), "accept".to_string())
        };

        PopupReqParam::new(Method::POST, url, map)
    }
}