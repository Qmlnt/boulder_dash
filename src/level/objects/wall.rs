use super::Obj;

pub struct Wall;

impl Obj for Wall {
    fn char(&self) -> &str {
        "🧱"
    }
}
