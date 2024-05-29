use super::{Dir, Level, Obj, Point, Request};

pub struct Rock;

impl Obj for Rock {
    fn char(&self) -> char {
        'O'
    }
    fn emoji(&self) -> char {
        '🪨'
    }
    fn name(&self) -> &str {
        "rock"
    }

    fn rock(&self) -> bool {
        true
    }

    fn tick(&self, level: &Level, (x, y): Point, _: Option<Dir>) -> Vec<Request> {
        if (x, y) == level.player || (x, y) == Dir::Up.apply_to(&level.player) {
            return vec![];
        }

        if level.get_obj((x, y + 1)).void() {
            return vec![Request::MoveObj((x, y), (x, y + 1))];
        }

        for side in [x - 1, x + 1] {
            if level.get_obj((side, y)).void() && level.get_obj((side, y + 1)).void() {
                return vec![Request::MoveObj((x, y), (side, y + 1))];
            }
        }

        vec![]
    }
}
