use enum_dispatch::enum_dispatch;
use std::error::Error;

mod cli;
mod gui;
mod tui;

use cli::Cli;
use gui::Gui;
use tui::Tui;

use crate::{
    args::{AppMode, Config},
    level::Level, //objects::Object,
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
    fn draw(&mut self, level: &mut Level, config: &Config) -> Result<(), Box<dyn Error>>;
}

#[enum_dispatch(Object)]
pub trait Labels: std::fmt::Debug {
    fn char(&self) -> char;
    fn emoji(&self) -> char;
    fn name(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}

pub fn get_mode(app_mode: &AppMode) -> Result<Mode, Box<dyn Error>> {
    Ok(match app_mode {
        AppMode::Gui => Gui::new()?.into(),
        AppMode::Tui => Tui::new().into(),
        AppMode::Cli => Cli::new().into(),
    })
}
