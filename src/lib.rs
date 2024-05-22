use std::error::Error;
use std::fs::read_link;
use std::{fs, process};
use std::{thread, time::Duration};

mod args;
mod draw;
mod input;
mod level;

// use args::AppMode;
pub use args::Config;
use input::Input;
use level::{Dir, Level};

fn read_level(path: &str) -> Result<Level, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let level = Level::parse(&contents)?;
    Ok(level)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut levels = Vec::new();
    for path in &config.level_paths {
        levels.push((read_level(path)?, path));
    }

    let input_receiver = input::init_term();

    for (mut level, path) in levels {
        let mut paused = false;

        draw::tui(&level.get_update())?;
        thread::sleep(Duration::from_secs(1));

        loop {
            thread::sleep(Duration::from_millis(config.delay));

            let mut direction = None;
            match input::read_term(&input_receiver) {
                Input::Unknown => (),
                Input::Quit => process::exit(0),
                Input::Reload => {
                    level = read_level(path)?;
                    draw::tui(&level.get_update())?;
                    thread::sleep(Duration::from_secs(1));
                }
                Input::Esc | Input::Space => paused = !paused,

                Input::Up => direction = Some(Dir::Up),
                Input::Down => direction = Some(Dir::Down),
                Input::Left => direction = Some(Dir::Left),
                Input::Right => direction = Some(Dir::Right),
            }

            if paused && direction.is_none() {
                continue;
            }

            level.tick(direction);
            let update = level.get_update();
            draw::tui(&update)?;

            if update.state.is_some() {
                break;
            }
        }
    }

    Ok(())
}
