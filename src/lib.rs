mod args;
mod direction;
mod editor;
mod game;
mod interaction;
mod objects;

type Point = (usize, usize); // (x, y)
pub use args::Config;
use args::ProgramMode;
use editor::Editor;
use game::{Level, State};

pub fn run(mut config: Config) -> Result<(), String> {
    let mut mode = interaction::get_mode(&config.app_mode)?;

    match config.program_mode {
        ProgramMode::Editor => {
            Editor::new(&config.level_paths[0])?.run(&mut config, &mut mode)?;
        }
        ProgramMode::Game => {
            for path in std::mem::take(&mut config.level_paths) {
                let mut level = Level::new(&path)?;

                let state = level.run(&mut config, &mut mode)?;
                if state == State::Lose {
                    break;
                }
            }
        }
    }

    Ok(())
}
