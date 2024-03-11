use ratatui::{prelude::*, widgets::*};
use tui_textarea::{TextArea, Input};

pub struct Popup<'a> {
    text_area: TextArea<'a>,
}

impl<'a> Popup<'a> {
    pub fn new(title: String) -> Popup<'a> {
        let mut text_area = TextArea::default();
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title)
            .style(Style::default().fg(Color::Blue)),
        );
        
        Popup {
            text_area,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = Rect {
            width: 80,
            height: 3,
            x: 42,
            y: 20,
        };

        frame.render_widget(self.text_area.widget(), area);
    }

    pub fn input(&mut self, input: Input) {
        self.text_area.input(input);
    }

    pub fn lines(&self) -> Vec<String> {
        self.text_area.lines().to_vec()
    }
}