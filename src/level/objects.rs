use super::{Direction, Level, Point, State};
use enum_dispatch::enum_dispatch;

mod dirt;
mod gem;
mod player;
mod rock;
mod void;
mod wall;

use dirt::Dirt;
use gem::Gem;
use player::Player;
use rock::Rock;
use void::Void;
use wall::Wall;

#[enum_dispatch]
#[derive(Debug)]
pub enum Object {
    Gem,
    Wall,
    Dirt,
    Rock,
    Void,
    Player,
}

pub enum Request {
    AddScore,
    AddMaxScore,
    UpdateState(State),
    MoveObj(Point, Point), // (from, to)
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

pub fn get_placeholder() -> Object {
    Void.into()
}

pub fn parse(chr: char) -> Result<Object, String> {
    Ok(match chr {
        'g' => Gem.into(),
        '#' => Wall.into(),
        'd' => Dirt.into(),
        'r' => Rock.into(),
        ' ' => Void.into(),
        'p' => Player.into(),
        _ => return Err(format!("Can't parse char `{chr}`")),
    })
}
