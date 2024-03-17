use std::collections::HashMap;

use reqwest::Method;
use ratatui::prelude::*;
use tui_textarea::Input;

pub enum PopupStatus {
    Continue,
    Cancel,
    Submit,
}

pub struct PopupReqParam {
    method : Method,
    map: HashMap<String, String>,
}

impl PopupReqParam {
    pub fn new(method: Method, map: HashMap<String, String>) -> Self {
        Self {
            method,
            map,
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn map(&self) -> HashMap<String, String> {
        self.map.clone()
    }
}

pub trait Popup {
    fn render(&mut self, frame: &mut Frame);
    fn input(&mut self, input: Input) -> PopupStatus;
    fn submit(&self) -> PopupReqParam;
}