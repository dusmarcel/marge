use std::{io, panic, time::Duration, ops::{Deref, DerefMut}};

use color_eyre::eyre::Result;

use crossterm::{
    cursor, 
    event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend as Backend, Terminal};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use futures::{FutureExt, StreamExt};

pub enum Event {
    Init,
    Error,
    Tick,
    Render,
    Key(KeyEvent),
    Mouse(MouseEvent),
    FocusGained,
    FocusLost,
    Resize(u16, u16),
    Paste(String),
    Quit,
}

pub struct Tui {
    terminal: Terminal<Backend<std::io::Stderr>>,
    frame_rate: f64,
    tick_rate: f64,
    cancellation_token: CancellationToken,
    task: JoinHandle<()>,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(Backend::new(std::io::stderr()))?;
        let frame_rate = 60.0;
        let tick_rate = 4.0;
        let cancellation_token = CancellationToken::new();
        let task = tokio::spawn(async {});
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Ok(Self {
            terminal,
            frame_rate,
            tick_rate,
            cancellation_token,
            task,
            event_rx,
            event_tx,
        })
    }

    pub fn start(&mut self) {
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        self.cancel();
        self.cancellation_token = CancellationToken::new();
        let _cancellation_token = self.cancellation_token.clone();
        let _event_tx = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            _event_tx.send(Event::Init).unwrap();
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _cancellation_token.cancelled() => {
                        break;
                    }
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    CrosstermEvent::Key(key) => {
                                        if key.kind == KeyEventKind::Press {
                                            _event_tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    CrosstermEvent::Mouse(mouse) => {
                                        _event_tx.send(Event::Mouse(mouse)).unwrap();
                                    },
                                    CrosstermEvent::Resize(x, y) => {
                                        _event_tx.send(Event::Resize(x, y)).unwrap();
                                    },
                                    CrosstermEvent::FocusLost => {
                                        _event_tx.send(Event::FocusLost).unwrap();
                                    },
                                    CrosstermEvent::FocusGained => {
                                        _event_tx.send(Event::FocusGained).unwrap();
                                    },
                                    CrosstermEvent::Paste(s) => {
                                        _event_tx.send(Event::Paste(s)).unwrap();
                                    }
                                }
                            }
                            Some(Err(_)) => {
                                _event_tx.send(Event::Error).unwrap();
                            }
                            None => {}
                        }
                    },
                    _ = tick_delay => {
                        _event_tx.send(Event::Tick).unwrap();
                    }
                    _ = render_delay => {
                        _event_tx.send(Event::Render).unwrap();
                    }
                }
            }
        });
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
            }
            if counter > 100 {
                eprintln!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }
        Ok(())
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stderr(),
            EnterAlternateScreen,
            cursor::Hide
        )?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.start();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            crossterm::execute!(
                std::io::stderr(),
                LeaveAlternateScreen,
                cursor::Show
            )?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    //pub fn draw(&mut self, marge: &mut Marge) -> Result<()> {
      //  self.terminal.draw(|frame| ui::render(rame, marge))?;
        //Ok(())
    //}

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.event_rx
            .recv()
            .await
            .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }

    fn reset() -> Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            cursor::Show
        )?;
        Ok(())
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    } 
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}