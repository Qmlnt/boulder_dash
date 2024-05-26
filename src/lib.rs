use std::{
    error::Error,
    fs, process,
    sync::mpsc::Receiver,
    thread,
    time::{Duration, Instant},
};

//extern crate sdl2;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;

mod args;
mod draw;
mod input;
mod level;

pub use args::Config;
use input::Input;
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

    let input_receiver = input::init_term();

    for (level, path) in levels {
        let state = run_level(&mut config, &input_receiver, level, &path)?;
        if state == State::Lose {
            break;
        }
    }

    Ok(())
}

pub fn run_level(
    config: &mut Config,
    input_rx: &Receiver<console::Key>,
    mut level: Level,
    path: &str,
) -> Result<State, Box<dyn Error>> {
    let mut launch_pause = true;
    let mut direction = None;
    let mut timer = Instant::now();

    loop {
        let update = &level.get_update();
        draw::draw(update, config)?;
        if let Some(state) = update.state {
            return Ok(state.clone());
        }

        thread::sleep(Duration::from_millis(10));

        match input::read_term(input_rx) {
            Input::Unknown => (),
            Input::Quit => process::exit(0),
            Input::Reload => {
                launch_pause = true;
                level = read_level(path)?;
                continue;
            }
            Input::DelayDown => {
                if config.delay >= 100 {
                    config.delay -= 50;
                }
            }
            Input::DelayUp => {
                if config.delay <= 950 {
                    config.delay += 50;
                }
            }
            Input::Esc | Input::Space => config.pause = !config.pause,

            Input::Up => direction = Some(Dir::Up),
            Input::Down => direction = Some(Dir::Down),
            Input::Left => direction = Some(Dir::Left),
            Input::Right => direction = Some(Dir::Right),
        }

        if timer.elapsed().as_millis() > u128::from(config.delay) {
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
