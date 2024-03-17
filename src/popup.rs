use std::collections::HashMap;

use reqwest::{Method, Url};
use ratatui::prelude::*;
use tui_textarea::Input;

pub enum PopupStatus {
    Continue,
    Cancel,
    Submit,
}

pub struct PopupReqParam {
    method : Method,
    url: Url,
    map: HashMap<String, String>,
}

impl PopupReqParam {
    pub fn new(method: Method, url: Url, map: HashMap<String, String>) -> Self {
        Self {
            method,
            url,
            map,
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn url(&self) -> Url {
        self.url.clone()
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