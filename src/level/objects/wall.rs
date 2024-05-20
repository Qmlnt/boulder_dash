use super::LevelObj;

pub struct Wall;

impl LevelObj for Wall {
    fn char(&self) -> &str {
        "🧱"
    }
}
