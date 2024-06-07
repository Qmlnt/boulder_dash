use super::{Input, Interaction, Tui};
use crate::{
    args::Config,
    level::{Labels, State, Update},
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

    fn draw(&mut self, level: Update, config: &Config) -> Result<(), Box<dyn Error>> {
        for (x, y) in level.damaged {
            self.tui.get_term().move_cursor_to(x, y)?;
            let char = level.matrix[y][x].char().to_string();
            self.tui.get_term().write_line(&char)?;
        }

        let status_msg = match level.state {
            Some(State::Win) => "You have won!".to_string(),
            Some(State::Lose) => "You have lost!".to_string(),
            None => format!(
                "\nScore: {}/{}   \nDelay: {}ms   \nPaused: {}   ",
                level.score,
                level.max_score,
                config.delay.as_millis(),
                if config.pause { "yes" } else { "no" }
            ),
        };

        self.tui.get_term().move_cursor_down(99)?;
        self.tui
            .get_term()
            .move_cursor_up(status_msg.lines().count())?;
        self.tui.get_term().write_line(&status_msg)?;

        Ok(())
    }
}
