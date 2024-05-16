use std::error::Error;
use std::{fs, process};
use std::{thread, time::Duration};

mod args;
mod draw;
mod input;
mod level;

use args::AppMode;
pub use args::Config;
use input::Input;
use draw::Draw;
use draw::Drawable;
// use level::Direction;
use level::Level;
use level::LevelUpdate;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let input_receiver = input::init_term();

    let mut levels = Vec::new();
    for level_path in &config.level_paths {
        let contents = fs::read_to_string(level_path)?;
        levels.push(Level::parse(&contents)?);
    }

    for mut level in levels {
        let mut paused = false;
        draw::tui(Draw::Init);

        loop {
            thread::sleep(Duration::from_millis(200));

            let mut direction = Direction::Idle;
            match input::read_term(&input_receiver) {
                Input::Idle => {}
                Input::Quit => process::exit(0),
                Input::Esc | Input::Space => paused = !paused,

                Input::Up => direction = Direction::Up,
                Input::Down => direction = Direction::Down,
                Input::Left => direction = Direction::Left,
                Input::Right => direction = Direction::Right,
            }

            if paused {
                continue;
            }

            // let level_state = level.tick(direction);
            //
            // if matches!(level_state, LevelUpdate::Win | LevelUpdate::Lose) {
            //     draw::tui(level_state);
            //     break;
            // }
            //
            // // out.clear_line().unwrap(); // TODO
            // // out.write_line(&format!("\nScore: {score}/{max_score}")) .unwrap();
            // draw::tui(level_state);
        }
    }

    Ok(())
}
