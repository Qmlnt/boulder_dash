use super::{Drawable, Input, Interaction, Tui};
use crate::objects::Labels;
use std::error::Error;

pub struct Cli {
    tui: Tui,
}

impl Cli {
    pub fn new() -> Self {
        let tui = Tui::default();
        tui.get_term().clear_screen().unwrap();

        Self { tui }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Interaction for Cli {
    fn get_input(&mut self) -> Input {
        self.tui.get_input()
    }

    fn draw(&mut self, drawable: &mut impl Drawable) -> Result<(), Box<dyn Error>> {
        let term = self.tui.get_term();

        for (x, y) in drawable.get_damaged() {
            if let Some(obj) = drawable.get_object((x, y)) {
                term.move_cursor_to(x, y)?;
                term.write_line(&obj.char().to_string())?;
            }
        }

        term.move_cursor_to(0, drawable.get_objects().len())?;
        term.clear_to_end_of_screen()?;
        term.move_cursor_down(1)?;
        term.write_line(&drawable.get_status())?;

        if let Some(&(x, y)) = drawable.get_cursor() {
            term.show_cursor()?;
            term.move_cursor_to(x, y)?;
        } else {
            term.hide_cursor()?;
        }

        Ok(())
    }
}
