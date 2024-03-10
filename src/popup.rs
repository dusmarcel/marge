use ratatui::{prelude::*, widgets::*};
use tui_textarea::TextArea;

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
            height: 5,
            x: 20,
            y: 20,
        };

        frame.render_widget(self.text_area.widget(), area);
    }
}