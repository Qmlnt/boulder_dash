use super::{Drawable, Input, Interaction, Tui};
use crate::{args::Config, objects::Labels};
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

    fn draw(
        &mut self,
        drawable: &mut impl Drawable,
        config: &Config,
    ) -> Result<(), Box<dyn Error>> {
        let term = self.tui.get_term();

        for (x, y) in drawable.get_damaged() {
            term.move_cursor_to(x, y)?;
            term.write_line(&drawable.get_object((x, y)).char().to_string())?;
        }

        let bottom = drawable.get_objects().len();
        let status = drawable.get_status(config);

        term.move_cursor_to(0, bottom + status.lines().count() + 1)?;
        term.clear_last_lines(status.lines().count())?;
        term.write_line(&status)?;

        if let Some(&(x, y)) = drawable.get_cursor() {
            term.show_cursor()?;
            term.move_cursor_to(x, y)?;
        } else {
            term.hide_cursor()?;
        }

        Ok(())
    }
}
