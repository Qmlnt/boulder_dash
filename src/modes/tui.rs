use super::{Input, Interaction, Labels};
use crate::{
    args::Config,
    level::{Level, State},
};
use console::{Key, Term};
use std::{error::Error, sync::mpsc, thread};

pub struct Tui {
    term: Term,
    input_rx: mpsc::Receiver<Key>,
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

impl Tui {
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel();

        let term = Term::stdout();
        let term_moved = term.clone();
        thread::spawn(move || loop {
            let key = term_moved.read_key().expect("Should always get a key");
            input_tx.send(key).expect("Receiver should be present");
        });

        term.hide_cursor().unwrap();

        Self { term, input_rx }
    }

    pub const fn get_term(&self) -> &Term {
        &self.term
    }
}

impl Interaction for Tui {
    fn get_input(&mut self) -> Input {
        let input = self.input_rx.try_recv();
        input.map_or(Input::Unknown, |key| match key {
            Key::Escape => Input::Esc,
            Key::Char(' ') => Input::Space,
            Key::Char('p') => Input::Reload,
            Key::Char(',') => Input::DelayDown,
            Key::Char('.') => Input::DelayUp,
            Key::Char('q') | Key::CtrlC => Input::Quit,
            Key::Char('w') | Key::ArrowUp => Input::Up,
            Key::Char('r') | Key::ArrowDown => Input::Down,
            Key::Char('a') | Key::ArrowLeft => Input::Left,
            Key::Char('s') | Key::ArrowRight => Input::Right,
            _ => Input::Unknown,
        })
    }

    fn draw(&mut self, level: &mut Level, config: &Config) -> Result<(), Box<dyn Error>> {
        self.term.clear_screen()?;

        for row in level.get_objects() {
            let mut line = String::new();
            for obj in row {
                line.push(obj.emoji());
            }
            self.term.write_line(&line)?;
        }

        let status_msg = match level.get_state() {
            Some(State::Win) => "You have won!".to_string(),
            Some(State::Lose) => "You have lost!".to_string(),
            None => format!(
                "\nScore: {}/{}\nDelay: {}ms\nPaused: {}",
                level.get_score(),
                level.get_max_score(),
                config.delay.as_millis(),
                if config.pause { "yes" } else { "no" }
            ),
        };
        self.term.write_line(&status_msg)?;

        Ok(())
    }
}
