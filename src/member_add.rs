use ratatui::{prelude::*, widgets::*};
use tui_textarea::{TextArea, Input};

use crate::popup::Popup;

pub struct MemberAdd<'a> {
    text_area: TextArea<'a>,
}

impl<'a> MemberAdd<'a> {
    pub fn new() -> MemberAdd<'a> {
        let mut text_area = TextArea::default();
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Add Member".to_string())
            .style(Style::default().fg(Color::Blue)),
        );

        MemberAdd {
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

    fn lines(&self) -> Vec<String> {
        self.text_area.lines().to_vec()
    }
}