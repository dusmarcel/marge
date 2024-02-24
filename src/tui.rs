use color_eyre::eyre::Result;

use crossterm::{cursor, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
//use ratatui;//::{Terminal, backend::CrosstermBackend as Backend};

pub struct Tui {
    //pub terminal: Terminal<Backend<std::io::Stderr>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        //let terminal = Terminal::new(Backend::new(std::io::stderr()))?;

        Ok(Self {
            //terminal,
        })
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stderr(),
            EnterAlternateScreen,
            cursor::Hide
        )?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            crossterm::execute!(
                std::io::stderr(),
                LeaveAlternateScreen,
                cursor::Show
            )?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }
}