use ratatui::prelude::*;
use tui_textarea::Input;

pub trait Popup {
    fn render(&mut self, frame: &mut Frame);
    fn input(&mut self, input: Input);
    fn lines(&self) -> Vec<String>;
}