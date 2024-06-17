use crate::{
    direction::Direction,
    game::level::{Level, Request, State},
    Point,
};
use enum_dispatch::enum_dispatch;

mod dirt;
mod gem;
mod player;
mod rock;
mod unknown;
mod void;
mod wall;

use dirt::Dirt;
use gem::Gem;
use player::Player;
use rock::Rock;
use unknown::Unknown;
use void::Void;
use wall::Wall;

#[enum_dispatch]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Gem,
    Wall,
    Dirt,
    Rock,
    Void,
    Player,
    Unknown,
}

impl Default for Object {
    fn default() -> Self {
        Void.into()
    }
}

impl Object {
    pub fn get_all_displayable() -> Vec<Self> {
        vec![
            Void.into(),
            Wall.into(),
            Rock.into(),
            Dirt.into(),
            Gem.into(),
            Player.into(),
        ]
    }

    pub fn new(chr: char) -> Self {
        match chr {
            '+' => Gem.into(),
            '#' => Wall.into(),
            '*' => Dirt.into(),
            'O' => Rock.into(),
            ' ' => Void.into(),
            'p' => Player.into(),
            _ => Unknown.into(),
        }
    }
}

#[enum_dispatch(Object)]
pub trait Labels: std::fmt::Debug {
    fn char(&self) -> char;
    fn emoji(&self) -> char;
    fn name(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}

#[enum_dispatch(Object)]
pub trait Properties {
    fn placeholder(&self) -> bool {
        false
    }
    fn can_be_moved(&self) -> bool {
        false
    }
    fn player(&self) -> bool {
        false
    }
    fn can_be_broken(&self) -> bool {
        false
    }
}

#[enum_dispatch(Object)]
pub trait Behaviour {
    fn init(&self) -> Vec<Request> {
        vec![]
    }
    fn on_broken(&self, _: &Level) -> Vec<Request> {
        vec![]
    }
    fn tick(&self, _: &Level, _: Point, _: Option<Direction>) -> Vec<Request> {
        vec![]
    }
}
