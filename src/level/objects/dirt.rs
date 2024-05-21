use super::Obj;

pub struct Dirt;

impl Obj for Dirt {
    fn broken_by_player(&self) -> bool {
        true
    }
    fn char(&self) -> &str {
        "d"
    }
}
