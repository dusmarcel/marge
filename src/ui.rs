use ratatui::{prelude::*, widgets::*};

#[derive(Copy, Clone)]
enum MenuItem {
    Domains,
    Lists,
    Messages,
    Configure,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Domains => 0,
            MenuItem::Lists => 1,
            MenuItem::Messages => 2,
            MenuItem::Configure => 3
        }        
    }
}
pub struct Ui {
    menu_titles: Vec<String>,
    active_menu_item: MenuItem,
}

impl Ui {
    pub fn new() -> Self {
        let menu_titles = vec![
            "Domains".to_string(),
            "Lists".to_string(),
            "Messages".to_string(),
            "Configure".to_string(),
            "Quit".to_string()];
        let active_menu_item = MenuItem::Domains;

        Self {
            menu_titles,
            active_menu_item,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                ]
                .as_ref(),
            )
            .split(area);

        let menu: Vec<Line> = self.menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Line::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::LightRed)),
                ])
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(self.active_menu_item.into())
            .block(Block::default()
                .title(" Marge ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Blue)))
                .highlight_style(Style::default().fg(Color::Red))
                .divider(Span::raw("|"));
        
        frame.render_widget(tabs, chunks[0]);
    }   
}