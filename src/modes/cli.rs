use super::{Input, Interaction, Labels, Tui};
use crate::{
    args::Config,
    level::{Level, State},
};
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

    fn draw(&mut self, level: &mut Level, config: &Config) -> Result<(), Box<dyn Error>> {
        for (x, y) in level.get_damaged() {
            self.tui.get_term().move_cursor_to(x, y)?;
            let char = level.get_object((x, y)).char().to_string();
            self.tui.get_term().write_line(&char)?;
        }

        let status_msg = match level.get_state() {
            Some(State::Win) => "You have won!".to_string(),
            Some(State::Lose) => "You have lost!".to_string(),
            None => format!(
                "\nScore: {}/{}   \nDelay: {}ms   \nPaused: {}   ",
                level.get_score(),
                level.get_max_score(),
                config.delay.as_millis(),
                if config.pause { "yes" } else { "no" }
            ),
        };

        self.tui.get_term().move_cursor_to(0, level.get_objects().len())?;
        self.tui.get_term().write_line(&status_msg)?;

        Ok(())
    }
}
