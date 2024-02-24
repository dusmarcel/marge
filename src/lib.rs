use color_eyre::eyre::Result;

mod tui;

use tui::Tui;

pub struct Marge {
    tui: Result<Tui>,
}

impl Marge {
    pub fn new() -> Self {
        let tui = Tui::new();

        Self {
            tui,
        }
    }

    pub async fn run(self) -> Result<()> {
        //let mut result = Ok(());
        
        if let Ok(mut tui) = self.tui {
            tui.enter()?;
            tui.exit()?;
            //result
        }
        //result?;

        //if let Ok(mut tui) = self.tui {
        //    result = tui.exit();
        //}
        //result?;

        Ok(())
    }
}