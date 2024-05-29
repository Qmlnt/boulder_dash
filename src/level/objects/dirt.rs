use super::Obj;

pub struct Dirt;

impl Obj for Dirt {
    fn breakable(&self) -> bool {
        true
    }
    fn char(&self) -> char {
        '*'
    }
    fn name(&self) -> &str {
        "dirt"
    }
    fn emoji(&self) -> char {
        '🟨'
    }
}
