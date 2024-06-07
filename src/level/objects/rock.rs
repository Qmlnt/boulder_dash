use super::{Behaviour, Direction, Labels, Level, Point, Properties, Request};

#[derive(Debug)]
pub struct Rock;

impl Labels for Rock {
    fn char(&self) -> char {
        'O'
    }
    fn emoji(&self) -> char {
        '🪨'
    }
}

impl Properties for Rock {
    fn can_be_moved(&self) -> bool {
        true
    }
}

impl Behaviour for Rock {
    fn tick(&self, level: &Level, (x, y): Point, _: Option<Direction>) -> Vec<Request> {
        if (x, y) == level.player || (x, y) == Direction::Up.apply_to(&level.player) {
            return vec![];
        }

        if level.get_obj((x, y + 1)).placeholder() {
            return vec![Request::MoveObj((x, y), (x, y + 1))];
        }

        for side in [x - 1, x + 1] {
            if level.get_obj((side, y)).placeholder() && level.get_obj((side, y + 1)).placeholder()
            {
                return vec![Request::MoveObj((x, y), (side, y + 1))];
            }
        }

        vec![]
    }
}
