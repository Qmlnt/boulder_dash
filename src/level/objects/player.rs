use super::{Obj, Request};

pub struct Player;

impl Obj for Player {
    fn player(&self) -> bool {
        true
    }
    fn on_broken(&self) -> Option<Request> {
        Some(Request::GameLost)
    }
    fn char(&self) -> &str {
        "🦀"
    }
}
