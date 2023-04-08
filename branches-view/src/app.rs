use std::io::{self, Write};
use std::path::PathBuf;

use chrono::{DateTime, Local, TimeZone};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor, Stylize};

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
    pub fn enter_refs_view(
        &mut self,
        path: &PathBuf,
    ) -> Result<ExitCode, Box<dyn std::error::Error>> {
        execute!(self.stdout, crossterm::terminal::EnterAlternateScreen)?;
        execute!(self.stdout, crossterm::cursor::MoveTo(0, 0))?;

        let repo = git2::Repository::open(path)?;

        let references: Vec<git2::Reference> = repo.references()?.filter_map(|r| r.ok()).collect();

        for reference in references.iter() {
            if let Ok(commit) = reference.peel_to_commit() {
                let timestamp = commit.time().seconds();
                let datetime: DateTime<Local> = Local.timestamp_opt(timestamp, 0).unwrap();
                let last_updated = datetime.format("%Y-%m-%d %H:%M:%S %:z");

                let author_name = commit.author().name().unwrap_or_default().to_string();

                let message = commit.summary().unwrap_or("").trim();
                let name = reference.name().unwrap_or("");

                queue!(self.stdout, Print('|'))?;
                queue!(
                    self.stdout,
                    SetForegroundColor(Color::Red),
                    Print(last_updated)
                )?;
                queue!(self.stdout, Print(' '))?;

                queue!(
                    self.stdout,
                    SetForegroundColor(Color::Green),
                    Print(author_name)
                )?;
                queue!(self.stdout, Print(' '))?;
                queue!(self.stdout, SetForegroundColor(Color::Blue), Print(name))?;
                queue!(self.stdout, Print(' '))?;
                queue!(
                    self.stdout,
                    SetForegroundColor(Color::White),
                    Print(message)
                )?;
                queue!(self.stdout, Print("\n"))?;
            }
        }

        self.stdout.flush()?;

        enable_raw_mode()?;

        execute!(self.stdout, cursor::MoveTo(0, 0))?;

        loop {
            match crossterm::event::read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) if code == KeyCode::Char('c') => break,
                Event::Key(KeyEvent { code, .. }) if code == KeyCode::Char('j') => {
                    // let (col, _) = cursor::position()?;
                    // if col == branches.len() as u16 {
                    //     continue;
                    // }

                    execute!(self.stdout, crossterm::cursor::MoveDown(1))?;
                }
                Event::Key(KeyEvent { code, .. }) if code == KeyCode::Char('k') => {
                    // let (col, _) = cursor::position()?;
                    // if col == 0 {
                    //     continue;
                    // }
                    execute!(self.stdout, crossterm::cursor::MoveUp(1))?;
                }
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

    fn max_word_length(words: Vec<&str>) -> usize {
        words.iter().map(|s| s.len()).max().unwrap_or(0)
    }

    fn padded_string(word: String, length: usize) -> String {
        let spaces_length = length - word.len();

        format!("{}{}", word, " ".repeat(spaces_length))
    }
}
