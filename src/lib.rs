use std::{
    error::Error,
    fs, process, thread,
    time::{Duration, Instant},
};

mod args;
mod interaction;
mod level;

pub use args::Config;
use interaction::{Gui, Input, Interaction, Mode, Tui};
use level::{Dir, Level, State};

fn read_level(path: &str) -> Result<Level, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let level = Level::parse(&contents)?;
    Ok(level)
}

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    let mut levels = Vec::new();
    for path in &config.level_paths {
        levels.push((read_level(path)?, path.clone()));
    }

    let mut mode = match config.app_mode {
        args::AppMode::Tui => Tui::new().into(),
        args::AppMode::Gui => Gui::new()?.into(),
    };

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
    mode: &mut Mode,
) -> Result<State, Box<dyn Error>> {
    let mut launch_pause = true;
    let mut direction = None;
    let mut timer = Instant::now();

    loop {
        let update = &level.get_update();
        mode.draw(update, config)?;

        if let Some(state) = update.state {
            return Ok(state.clone());
        }

        thread::sleep(Duration::from_millis(10));

        let inp = mode.get_input();
        match inp {
            Input::Unknown => (),
            Input::Quit => process::exit(0),
            Input::Reload => {
                launch_pause = true;
                level = read_level(level_path)?;
                continue;
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

            Input::Up => direction = Some(Dir::Up),
            Input::Down => direction = Some(Dir::Down),
            Input::Left => direction = Some(Dir::Left),
            Input::Right => direction = Some(Dir::Right),
        }

        if timer.elapsed() > config.delay {
            timer = Instant::now();

            if direction.is_some() {
                launch_pause = false;
            }

            if (config.pause && direction.is_none()) || launch_pause {
                continue;
            }

            level.tick(direction);
            direction = None;
        }
    }
}
