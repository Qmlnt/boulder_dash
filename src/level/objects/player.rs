use super::Obj;

pub struct Player;

impl Obj for Player {
    fn player(&self) -> bool {
        true
    }
    fn char(&self) -> char {
        '🦀'
    }
}
