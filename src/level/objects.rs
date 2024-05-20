use enum_dispatch::enum_dispatch;

mod dirt;
mod gem;
mod player;
mod rock;
mod wall;

use dirt::Dirt;
use gem::Gem;
use player::Player;
use rock::Rock;
use wall::Wall;

#[enum_dispatch]
pub enum LevelObject {
    Gem,
    Wall,
    Dirt,
    Rock,
    Player,
}

pub enum ObjRequest {
    AddScore,
    AddMaxScore,
    GameLost,
}

#[enum_dispatch(LevelObject)]
pub trait LevelObj {
    fn char(&self) -> &str;

    fn init(&self) -> Option<ObjRequest> {
        None
    }
    fn on_broken(&self) -> Option<ObjRequest> {
        None
    }

    fn rock(&self) -> bool {
        false
    }
    fn player(&self) -> bool {
        false
    }
    fn broken_by_player(&self) -> bool {
        false
    }
}

pub fn parse(chr: char) -> Result<LevelObject, String> {
    Ok(match chr {
        'g' => Gem.into(),
        '#' => Wall.into(),
        'd' => Dirt.into(),
        'r' => Rock.into(),
        'p' => Player.into(),
        _ => return Err(format!("Can't parse char `{chr}`")),
    })
}
