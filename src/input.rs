use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::App;

enum Focus {
    Normal,
    Editing,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    Focus::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = Focus::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    Focus::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = Focus::Normal,
                        _ => {}
                    },
                    Focus::Editing => {}
                }
            }
        }
    }
}
