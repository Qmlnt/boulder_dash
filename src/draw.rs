use console::Term;
use std::error::Error;

use crate::{
    args::AppMode,
    level::{Obj, State, Update},
    Config,
};

pub fn draw(level: &Update, config: &Config) -> Result<(), Box<dyn Error>> {
    match config.app_mode {
        AppMode::Tui => {
            let out = Term::stdout();

            out.clear_screen()?;
            out.hide_cursor()?;

            for row in level.matrix {
                let mut line = String::new();
                for obj in row {
                    line.push(obj.char());
                }
                out.write_line(&line)?;
            }

            let status_msg = match level.state {
                Some(State::Win) => "You have won!".to_string(),
                Some(State::Lose) => "You have lost!".to_string(),
                None => format!(
                    "\nScore: {}/{}\nDelay: {}ms\nPaused: {}",
                    level.score,
                    level.max_score,
                    config.delay,
                    if config.pause { "yes" } else { "no" }
                ),
            };

            out.write_line(&status_msg)?;
        }
        AppMode::Gui => {
            todo!()
        }
    }

    Ok(())
}

// pub fn tui(level: &level::Update, delay: u16, paused: bool) -> io::Result<()> {
// out.move_cursor_down(999)?; // bottom
// out.write_line(&lines)?;
// out.move_cursor_up(1)?;
// out.move_cursor_to(0, 0)?;
// out.move_cursor_to(x, y)?;
// out.write_line(obj.map_or(" ", |o| o.char()))?;

// Ok(())
// }
