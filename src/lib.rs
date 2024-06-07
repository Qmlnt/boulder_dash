use std::{
    error::Error,
    fs, process, thread,
    time::{Duration, Instant},
};

mod args;
mod level;
mod modes;
mod objects;

pub use args::Config;
use level::{Direction, Level, State};
use modes::{Input, Interaction, Mode};

fn read_level(path: &str) -> Result<Level, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let level = Level::parse(&contents)?;
    Ok(level)
}

// TODO level drawer
pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    let mut levels = Vec::new();
    for path in &config.level_paths {
        levels.push((read_level(path)?, path.clone()));
    }

    let mut mode = modes::get_mode(&config.app_mode)?;

    for (level, path) in levels {
        let state = run_level(level, &path, &mut config, &mut mode)?;
        if state == State::Lose {
            break;
        }
    }

    Ok(())
}

pub fn run_level(
    mut level: Level,
    level_path: &str,
    config: &mut Config,
    display_mode: &mut Mode,
) -> Result<State, Box<dyn Error>> {
    let mut launch_pause = true;
    let mut direction = None;
    let mut timer = Instant::now();

    display_mode.draw(&mut level, config)?;

    loop {
        let mut input = true;
        match display_mode.get_input() {
            Input::Unknown => input = false,
            Input::Quit => process::exit(0),
            Input::Reload => {
                launch_pause = true;
                level = read_level(level_path)?;
            }
            Input::DelayDown => {
                if config.delay.as_millis() >= 100 {
                    config.delay -= Duration::from_millis(50);
                }
            }
            Input::DelayUp => {
                if config.delay.as_millis() <= 950 {
                    config.delay += Duration::from_millis(50);
                }
            }
            Input::Esc | Input::Space => config.pause = !config.pause,

            Input::Up => direction = Some(Direction::Up),
            Input::Down => direction = Some(Direction::Down),
            Input::Left => direction = Some(Direction::Left),
            Input::Right => direction = Some(Direction::Right),
        }

        if timer.elapsed() > config.delay {
            timer = Instant::now();

            if launch_pause && direction.is_some() {
                launch_pause = false;
            }
            if (config.pause && direction.is_none()) || launch_pause {
                continue;
            }

            level.tick(direction.take());
            display_mode.draw(&mut level, config)?;

            if let Some(state) = level.get_state() {
                return Ok(state.clone());
            }
        } else if input {
            display_mode.draw(&mut level, config)?;
        }

        thread::sleep(Duration::from_millis(10));
    }
}
