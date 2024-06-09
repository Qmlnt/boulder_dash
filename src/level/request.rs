use super::{Point, State};

pub enum Request {
    AddScore,
    AddMaxScore,
    UpdateState(State),
    MoveObj(Point, Point), // (from, to)
}
