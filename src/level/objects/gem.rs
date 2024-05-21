use super::{Obj, Request};

pub struct Gem;

impl Obj for Gem {
    fn breakable(&self) -> bool {
        true
    }
    fn init(&self) -> Option<Request> {
        Some(Request::AddMaxScore)
    }
    fn on_broken(&self) -> Option<Request> {
        Some(Request::AddScore)
    }
    fn char(&self) -> &str {
        "💎"
    }
}
