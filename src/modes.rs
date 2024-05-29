use enum_dispatch::enum_dispatch;
use std::error::Error;

mod cli;
mod gui;
mod tui;

use cli::Cli;
use gui::Gui;
use tui::Tui;

use super::{
    args::{AppMode, Config},
    level::Update,
};

#[enum_dispatch]
pub enum Mode {
    Gui,
    Tui,
    Cli,
}

#[derive(PartialEq, Eq)]
pub enum Input {
    Esc,
    Quit,
    Reload,
    Unknown,

    Up,
    Down,
    Left,
    Right,
    Space,
    DelayDown,
    DelayUp,
}

#[enum_dispatch(Mode)]
pub trait Interaction {
    fn get_input(&mut self) -> Input;
    fn draw(&mut self, level: Update, config: &Config) -> Result<(), Box<dyn Error>>;
}

pub fn get_mode(app_mode: &AppMode) -> Result<Mode, String> {
    Ok(match app_mode {
        AppMode::Gui => Gui::new()?.into(),
        AppMode::Tui => Tui::new().into(),
        AppMode::Cli => Cli::new().into(),
    })
}
