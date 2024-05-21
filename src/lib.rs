use std::error::Error;
use std::{fs, process};
use std::{thread, time::Duration};

mod args;
mod draw;
mod input;
mod level;

use args::AppMode;
pub use args::Config;
use draw::Draw;
use input::Input;
use level::{Dir, Level, Obj, Object, Update};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let input_receiver = input::init_term();

    let mut levels = Vec::new();
    for level_path in &config.level_paths {
        let contents = fs::read_to_string(level_path)?;
        levels.push(Level::parse(&contents)?);
    }

    for mut level in levels {
        let mut paused = false;
        draw::tui(Draw::Init)?;
        draw::tui(Draw::Damaged(level.get_update().damaged))?;

        loop {
            thread::sleep(Duration::from_millis(1000));

            let mut direction = None;
            match input::read_term(&input_receiver) {
                Input::Unknown => (),
                Input::Quit => process::exit(0),
                Input::Esc | Input::Space => paused = !paused,

                Input::Up => direction = Some(Dir::Up),
                Input::Down => direction = Some(Dir::Down),
                Input::Left => direction = Some(Dir::Left),
                Input::Right => direction = Some(Dir::Right),
            }

            if paused {
                continue;
            }

            let Update {
                score,
                max_score,
                state,
                damaged,
            } = level.tick(direction);

            if damaged.is_empty() {
                continue;
            }

            draw::tui(Draw::Damaged(damaged))?;
             // draw::tui(Draw::Status(format!("\nScore: {score}/{max_score}")));

            if state.is_some() {
                todo!();
            }
        }
    }

    Ok(())
}
