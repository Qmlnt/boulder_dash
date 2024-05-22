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
    GameLost,
}

#[enum_dispatch(Object)]
pub trait Obj {
    fn char(&self) -> char;

    fn init(&self) -> Option<Request> {
        None
    }
    fn on_broken(&self) -> Option<Request> {
        None
    }

    fn rock(&self) -> bool {
        false
    }

    fn breakable(&self) -> bool {
        false
    }
    fn void(&self) -> bool {
        false
    }
    fn player(&self) -> bool {
        false
    }
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
