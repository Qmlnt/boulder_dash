use super::LevelObj;

pub struct Dirt;

impl LevelObj for Dirt {
    fn broken_by_player(&self) -> bool {
        true
    }
    fn char(&self) -> &str {
        "🟨"
    }
}
