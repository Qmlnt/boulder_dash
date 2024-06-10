use crate::{
    level::{Behaviour, Level, Point, Properties, Request, State},
    modes::Labels,
    Direction,
};
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
