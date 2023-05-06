use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, SetTitle};
use std::io;
use tui::backend::Backend;
use tui::Terminal;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn init(&mut self, app: &App) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        if let Ok(Some(v)) = app.tts.voice() {
            crossterm::execute!(
                io::stderr(),
                SetTitle(format!(
                    "{}, {:?}, {}\n",
                    v.name(),
                    v.gender().unwrap(),
                    v.language()
                )),
                EnterAlternateScreen,
                EnableMouseCapture
            )?;
        }
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
