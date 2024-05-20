use super::{LevelObj, ObjRequest};

pub struct Player;

impl LevelObj for Player {
    fn player(&self) -> bool {
        true
    }
    fn on_broken(&self) -> Option<ObjRequest> {
        Some(ObjRequest::GameLost)
    }
    fn char(&self) -> &str {
        "🦀"
    }
}
