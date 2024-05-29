use super::Obj;

pub struct Wall;

impl Obj for Wall {
    fn char(&self) -> char {
        '#'
    }
    fn emoji(&self) -> char {
        '🧱'
    }
    fn name(&self) -> &str {
        "wall"
    }
}
