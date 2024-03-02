use ratatui::{prelude::*, widgets::*};

#[derive(Copy, Clone)]
enum MenuItem {
    Domains,
    Lists,
    Members,
    Messages,
    Configure,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Domains => 0,
            MenuItem::Lists => 1,
            MenuItem::Members => 2,
            MenuItem::Messages => 3,
            MenuItem::Configure => 4
        }        
    }
}
pub struct Ui {
    menu_titles: Vec<String>,
    active_menu_item: MenuItem,
    list_vec:Vec<String>,
    state: ListState,
    status: String,
}

impl Ui {
    pub fn new() -> Self {
        let menu_titles = vec![
            "Domains".to_string(),
            "Lists".to_string(),
            "Members".to_string(),
            "Messages".to_string(),
            "Configure".to_string(),
            "Quit".to_string()];
        let active_menu_item = MenuItem::Domains;
        let list_vec = vec!["waiting".to_string()];
        let state = ListState::default();
        let status = String::new();

        Self {
            menu_titles,
            active_menu_item,
            list_vec,
            state,
            status,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(4)
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

        let list = List::new(self.list_vec.clone());

        frame.render_stateful_widget(list, chunks[1], &mut self.state);

        let status = Paragraph::new(format!("{}", self.status))
            .block(Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Blue)))
            .fg(Color::LightRed)
            .wrap(Wrap{ trim: false});
        
        frame.render_widget(status, chunks[2]);
    }

    pub fn set_list_vec(&mut self, list_vec: Vec<String>) {
        self.list_vec = list_vec;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }   
}