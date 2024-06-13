use crate::{
    args::{AppMode, Config},
    objects::Object,
    Point,
};
use enum_dispatch::enum_dispatch;
use std::{collections::HashSet, error::Error};

mod cli;
mod gui;
mod tui;

use cli::Cli;
use gui::Gui;
use tui::Tui;

#[derive(PartialEq, Eq)]
pub enum Input {
    Esc,
    Unknown,
    Q,
    R,
    W,
    A,
    S,
    D,

    Up,
    Down,
    Left,
    Right,
    Space,
    Comma,
    Period,
}

#[enum_dispatch]
pub enum Mode {
    Gui,
    Tui,
    Cli,
}

#[enum_dispatch(Mode)]
pub trait Interaction {
    fn get_input(&mut self) -> Input;
    fn draw(&mut self, drawable: &mut impl Drawable, config: &Config)
        -> Result<(), Box<dyn Error>>;
}

pub trait Drawable {
    fn get_cursor(&self) -> Option<&Point> {
        None
    }
    fn get_damaged(&mut self) -> HashSet<Point>;
    fn get_objects(&self) -> &Vec<Vec<Object>>;
    fn get_object(&self, point: Point) -> &Object;
    fn get_status(&self, config: &Config) -> String;
}

pub fn get_mode(app_mode: &AppMode) -> Result<Mode, String> {
    Ok(match app_mode {
        AppMode::Gui => Gui::new().map_err(|e| e.to_string())?.into(),
        AppMode::Tui => Tui::new().into(),
        AppMode::Cli => Cli::new().into(),
    })
}
