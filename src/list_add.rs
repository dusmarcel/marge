use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};
use reqwest::{Method, Url};
use tui_textarea::{TextArea, Input, Key};

use crate::{config::Config, popup::{Popup, PopupStatus, PopupReqParam}};

#[derive(Clone)]
pub struct ListAdd<'a> {
    config: Config,
    text_area: TextArea<'a>,
}

impl<'a> ListAdd<'a> {
    pub fn new(config: Config) -> Self {
        let mut text_area = TextArea::default();
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Add List ".to_string())
            .style(Style::default().fg(Color::Blue)),
        );

        Self {
            config,
            text_area,
        }
    }
}

impl Popup for ListAdd<'_> {
    fn render(&mut self, frame: &mut Frame) {
        let area = Rect {
            width: 80,
            height: 3,
            x: 42,
            y: 20,
        };

        frame.render_widget(&self.text_area, area);
    }

    fn input(&mut self, input: Input) -> PopupStatus {
        let mut status = PopupStatus::Continue;
        match input {
            Input { key: Key::Esc, .. } => status = PopupStatus::Cancel,
            Input { key: Key::Enter, .. } => status = PopupStatus::Submit,
            input => { self.text_area.input(input); }
        }

        status
    }

    fn submit(&self) -> PopupReqParam {
        let url = Url::parse(&format!("{}://{}:{}/3.1/lists",
            self.config.protocol(),
            self.config.host(),
            self.config.port())).unwrap();

        let mut map = HashMap::new();
        map.insert("fqdn_listname".to_string(), self.text_area.lines()[0].clone());

        PopupReqParam::new(Method::POST, url, map)
    }
}