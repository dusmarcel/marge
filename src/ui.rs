use ratatui::{prelude::*, widgets::*};

#[derive(Copy, Clone)]
pub enum MenuItem {
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
    list_vec: Vec<String>,
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
                if *t == "Messages".to_string() {
                    Line::from(vec![
                        Span::styled("Me", Style::default().fg(Color::LightRed)),
                        Span::styled("s", Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED)),
                        Span::styled("sages", Style::default().fg(Color::LightRed))
                    ])
                } else {
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
                }
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(self.active_menu_item.into())
            .block(Block::default()
                .title(" Marge ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Blue)))
            .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .divider(Span::raw("|"));
        
        frame.render_widget(tabs, chunks[0]);

        let style = Style::default().fg(Color::Blue);
        let lv: Vec<Line<'_>> = self.list_vec.iter().map(|s| {
            Line::styled(s.clone(), style)
        }).collect();
        let list = List::new(lv)
            .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        if self.selected() == None && self.list_vec.len() > 0 {
            self.state.select(Some(0));
        }

        frame.render_stateful_widget(list, chunks[1], &mut self.state);

        let status = Paragraph::new(format!("{}", self.status))
            .block(Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Blue)))
            .fg(Color::LightRed)
            .wrap(Wrap{ trim: false });
        
        frame.render_widget(status, chunks[2]);
    }

    pub fn set_active_menu_item(&mut self, menu_item: MenuItem) {
        self.active_menu_item = menu_item;
    }

    pub fn set_list_vec(&mut self, list_vec: Vec<String>) {
        self.list_vec = list_vec;
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list_vec.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
    }

    pub fn up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list_vec.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0
        };
        self.state.select(Some(i));
    }
}