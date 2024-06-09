use super::{Direction, Level, Point, Request};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Object)]
pub trait Properties {
    fn placeholder(&self) -> bool {
        false
    }
    fn can_be_moved(&self) -> bool {
        false
    }
    fn player(&self) -> bool {
        false
    }
    fn can_be_broken(&self) -> bool {
        false
    }
}

#[enum_dispatch(Object)]
pub trait Behaviour {
    fn init(&self) -> Vec<Request> {
        vec![]
    }
    fn on_broken(&self, _: &Level) -> Vec<Request> {
        vec![]
    }
    fn tick(&self, _: &Level, _: Point, _: Option<Direction>) -> Vec<Request> {
        vec![]
    }
}
