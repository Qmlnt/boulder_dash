use crate::{
    args::{Arguments, InteractionMode},
    objects::Object,
    Point,
};
use enum_dispatch::enum_dispatch;
use std::{cmp, error::Error};

mod cli;
mod gui;
mod tui;

use cli::Cli;
use gui::Gui;
use tui::Tui;

#[derive(Clone, PartialEq, Eq)]
pub enum Input {
    Quit,
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
    fn draw(&mut self, drawable: &mut impl Drawable) -> Result<(), Box<dyn Error>>;
}

pub trait Drawable {
    fn get_cursor(&self) -> Option<&Point> {
        None
    }
    fn get_width(&self) -> usize {
        cmp::max(
            self.get_objects()
                .iter()
                .max_by_key(|r| r.len())
                .map_or(0, Vec::len),
            self.get_status()
                .lines()
                .max_by_key(|r| r.len())
                .map_or(0, |s| s.len() * 3 / 5),
        )
    }
    fn get_height(&self) -> usize {
        self.get_objects().len() + self.get_status().lines().count()
    }
    fn get_status(&self) -> String;
    fn get_damaged(&mut self) -> Vec<Point>;
    fn get_objects(&self) -> &Vec<Vec<Object>>;
    fn get_object(&self, point: Point) -> Option<&Object>;
}

pub fn get_mode(args: &Arguments) -> Result<Mode, String> {
    Ok(match args.interaction_mode {
        InteractionMode::Gui => Gui::new(args.size).map_err(|e| e.to_string())?.into(),
        InteractionMode::Tui => Tui::new().into(),
        InteractionMode::Cli => Cli::new().into(),
    })
}
