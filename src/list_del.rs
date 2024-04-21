use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};
use reqwest::{Method, Url};
use tui_textarea::{Input, Key};

use crate::{config::Config, popup::{Popup, PopupReqParam, PopupStatus}};

#[derive(Clone)]
pub struct ListDel<'a> {
    config: Config,
    paragraph: Paragraph<'a>,
}

impl<'a> ListDel<'a> {
    pub fn new(config: Config) -> Self {
        let line = Line::raw("Are you sure? Type 'y' or Enter for yes or 'n' or Esc for no");
        let paragraph = Paragraph::new(line)
            .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Delete List? ".to_string())
            .style(Style::default().fg(Color::Blue)),
        );

        Self {
            config,
            paragraph,
        }
    }
}

impl Popup for ListDel<'_> {
    fn render(&mut self, frame: &mut Frame) {
        let area = Rect {
            width: 62,
            height: 3,
            x: 42,
            y: 20,
        };

        frame.render_widget(self.paragraph.clone(), area);
    }

    fn input(&mut self, input: Input) -> PopupStatus {
        let mut status = PopupStatus::Continue;
        match input {
            Input { key: Key::Esc, .. } |
            Input { key: Key::Char('n'), .. } |
            Input { key: Key::Char('N'), .. } => status = PopupStatus::Cancel,
            Input { key: Key::Enter, .. } |
            Input { key: Key::Char('y'), .. } |
            Input { key: Key::Char('Y'), .. } => status = PopupStatus::Submit,
            _input => {}
        }

        status
    }

    fn submit(&self) -> PopupReqParam {
        let url = Url::parse(&format!("{}://{}:{}/3.1/lists/{}",
            self.config.protocol(),
            self.config.host(),
            self.config.port(),
            self.config.list().unwrap().list_id())).unwrap();
        let map = HashMap::new();

        PopupReqParam::new(Method::DELETE, url, map)
    }
}