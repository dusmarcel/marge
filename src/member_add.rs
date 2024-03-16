use std::collections::HashMap;

//use color_eyre::config;
use ratatui::{prelude::*, widgets::*};
use reqwest::Method;
use tui_textarea::{TextArea, Input};

use crate::{config::Config, popup::{Popup, PopupReqParam}};

#[derive(Clone)]
pub struct MemberAdd<'a> {
    config: Config,
    text_area: TextArea<'a>,
}

impl<'a> MemberAdd<'a> {
    pub fn new(config: Config) -> MemberAdd<'a> {
        let mut text_area = TextArea::default();
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Add Member".to_string())
            .style(Style::default().fg(Color::Blue)),
        );

        MemberAdd {
            config,
            text_area,
        }
    }
}

impl Popup for MemberAdd<'_> {
    fn render(&mut self, frame: &mut Frame) {
        let area = Rect {
            width: 80,
            height: 3,
            x: 42,
            y: 20,
        };

        frame.render_widget(self.text_area.widget(), area);
    }

    fn input(&mut self, input: Input) {
        self.text_area.input(input);
    }

    //fn lines(&self) -> Vec<String> {
    //    self.text_area.lines().to_vec()
    //}

    fn submit(&self) -> PopupReqParam {
        let mut map = HashMap::new();
        map.insert("list_id".to_string(), self.config.list().unwrap().list_id());
        map.insert("subscriber".to_string(), self.text_area.lines()[0].clone());
        map.insert("display_name".to_string(), "".to_string());
        map.insert("pre_verified".to_string(), "true".to_string());
        map.insert("pre_confirmed".to_string(), "true".to_string());
        map.insert("pre_approved".to_string(), "true".to_string());
        map.insert("send_welcome_message".to_string(), "false".to_string());

        PopupReqParam::new(Method::POST, map)
    }
}