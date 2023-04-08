use std::io::{self, Write};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, queue, style::Print};

use crate::exit_code::ExitCode;

pub struct App {
    stdout: io::Stdout,
}

impl App {
    pub fn new() -> Self {
        App {
            stdout: io::stdout(),
        }
    }
}

impl App {
    pub fn enter_alternate_screen(&mut self) -> Result<ExitCode, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        execute!(self.stdout, crossterm::terminal::EnterAlternateScreen)?;
        execute!(self.stdout, crossterm::cursor::MoveTo(0, 0))?;
        execute!(self.stdout, Print("Press 'q' to exit."))?;

        loop {
            match crossterm::event::read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) if code == KeyCode::Char('c') => break,
                Event::Key(KeyEvent { code, .. }) if code == KeyCode::Char('q') => break,
                Event::Key(KeyEvent { code, .. }) if code == KeyCode::Enter => {
                    execute!(self.stdout, Clear(ClearType::All))?;
                    execute!(self.stdout, crossterm::cursor::MoveTo(0, 0))?;
                }
                _ => {}
            }
        }

        execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(ExitCode::Success)
    }
}
