use super::LevelObj;

pub struct Rock;

impl LevelObj for Rock {
    fn rock(&self) -> bool {
        true
    }
    fn char(&self) -> &str {
        "🪨"
    }
}
