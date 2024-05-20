use super::{LevelObj, ObjRequest};

pub struct Gem;

impl LevelObj for Gem {
    fn broken_by_player(&self) -> bool {
        true
    }
    fn on_broken(&self) -> Option<ObjRequest> {
        Some(ObjRequest::AddScore)
    }
    fn char(&self) -> &str {
        "💎"
    }
}
