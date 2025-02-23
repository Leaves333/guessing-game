use std::cmp::Ordering;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::random_range;
use ratatui::DefaultTerminal;

use crate::App;

pub enum Focus {
    Normal,
    Editing,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.focus {
                    Focus::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.focus = Focus::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    Focus::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_answer(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.focus = Focus::Normal,
                        _ => {}
                    },
                    Focus::Editing => {}
                }
            }
        }
    }

    fn submit_answer(&mut self) {
        match self.input.parse::<i32>() {
            Ok(guess) => {
                self.previous_guesses.push(guess);
                self.deviations.push(guess - self.hidden_number);

                let response = match guess.cmp(&self.hidden_number) {
                    Ordering::Less => "too low!",
                    Ordering::Greater => "too high!",
                    Ordering::Equal => {
                        self.hidden_number = random_range(0..100);
                        "yay you did it! picking a new number... "
                    }
                };
                let result_message = format!("{guess}: {response}");
                self.messages.push(result_message);
            }
            Err(_) => {}
        };

        self.input.clear();
        self.reset_cursor();
    }
}
