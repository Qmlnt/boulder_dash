use enum_dispatch::enum_dispatch;
use std::path::Path;

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

use super::{Dir, Level, Point, State};
use sdl2::{image::LoadSurface, surface::Surface};

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
    UpdateState(State),
    MoveObj(Point, Point),
}

#[enum_dispatch(Object)]
pub trait Obj {
    fn char(&self) -> char;
    fn emoji(&self) -> char;
    fn name(&self) -> &str;

    fn void(&self) -> bool {
        false
    }
    fn rock(&self) -> bool {
        false
    }
    fn player(&self) -> bool {
        false
    }
    fn breakable(&self) -> bool {
        false
    }

    // TODO that's messed up
    fn sprite(&self) -> Result<Surface, String> {
        Surface::from_file(Path::new(&format!("assets/sprites/{}.png", self.name())))
    }

    fn init(&self) -> Vec<Request> {
        vec![]
    }
    fn on_broken(&self, _: &Level) -> Vec<Request> {
        vec![]
    }
    fn tick(&self, _: &Level, _: Point, _: Option<Dir>) -> Vec<Request> {
        vec![]
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
