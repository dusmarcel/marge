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
    sel_domain: Option<String>,
    sel_list: Option<String>,
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
        let sel_domain = None;
        let sel_list = None;
        let list_vec = vec!["waiting".to_string()];
        let mut state = ListState::default();
        state.select(Some(0));
        let status = String::new();

        Self {
            menu_titles,
            active_menu_item,
            sel_domain,
            sel_list,
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
                    Constraint::Length(1),
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

        let mut domain = "(None)";
        if let Some(d) = &self.sel_domain {
            domain = d;
        }
        let mut list = "(None)";
        if let Some(l) = &self.sel_list {
            list = l
        }
        let header = Paragraph::new(format!("Selected domain: {} || Selected list: {}", domain, list))
            .style(Style::default().fg(Color::LightRed));

        frame.render_widget(header, chunks[1]);

        let style = Style::default().fg(Color::Blue);
        let lv: Vec<Line<'_>> = self.list_vec.iter().map(|s| {
            Line::styled(s.clone(), style)
        }).collect();
        let list = List::new(lv)
            .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, chunks[2], &mut self.state);

        let status = Paragraph::new(format!("{}", self.status))
            .block(Block::default()
                .title(" Status ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Blue)))
            .fg(Color::LightRed)
            .wrap(Wrap{ trim: false });
        
        frame.render_widget(status, chunks[3]);
    }

    pub fn set_active_menu_item(&mut self, menu_item: MenuItem) {
        self.active_menu_item = menu_item;
    }

    pub fn set_sel_domain(&mut self, sel_domain: Option<String>) {
        self.sel_domain = sel_domain;
    }

    pub fn set_sel_list(&mut self, sel_list: Option<String>) {
        self.sel_list = sel_list;
    }

    pub fn set_list_vec(&mut self, list_vec: Vec<String>) {
        self.list_vec = list_vec;
        self.state.select(Some(0));
    }

    pub fn select(&mut self, i: Option<usize>) {
        self.state.select(i)
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