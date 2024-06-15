use super::{Drawable, Input, Interaction};
use crate::objects::Labels;
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
            Key::Char(',') => Input::Comma,
            Key::Char('.') => Input::Period,
            Key::Char('q') => Input::Q,
            Key::Char('p') => Input::R,

            Key::Char('w') => Input::W,
            Key::Char('a') => Input::A,
            Key::Char('r') => Input::S,
            Key::Char('s') => Input::D,
            Key::ArrowUp => Input::Up,
            Key::ArrowDown => Input::Down,
            Key::ArrowLeft => Input::Left,
            Key::ArrowRight => Input::Right,

            _ => Input::Unknown,
        })
    }

    fn draw(&mut self, drawable: &mut impl Drawable) -> Result<(), Box<dyn Error>> {
        self.term.clear_screen()?;

        drawable.get_damaged(); // Empty damaged buffer
        for row in drawable.get_objects() {
            let mut line = String::new();
            for obj in row {
                line.push(obj.emoji());
            }
            self.term.write_line(&line)?;
        }

        self.term.move_cursor_down(1)?;
        self.term.write_line(&drawable.get_status())?;

        if let Some(&(x, y)) = drawable.get_cursor() {
            self.term.show_cursor()?;
            self.term.move_cursor_to(x, y)?;
        } else {
            self.term.hide_cursor()?;
        }

        Ok(())
    }
}
