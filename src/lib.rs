mod args;
mod direction;
mod editor;
mod game;
mod interaction;
mod objects;

pub use args::Arguments;
use args::ProgramMode;
use editor::Editor;
use game::Game;
use std::error::Error;

type Point = (usize, usize); // (x, y)

pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
    let mut mode = interaction::get_mode(args)?;

    match args.program_mode {
        ProgramMode::Game => Game::new(args)?.run(&mut mode),
        ProgramMode::Editor => Editor::new(args)?.run(&mut mode),
    }
}
