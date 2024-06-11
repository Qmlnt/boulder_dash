use super::{Behaviour, Direction, Labels, Level, Point, Properties, Request};

#[derive(Debug, Clone, PartialEq)]
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
        if (x, y) == *level.get_player()
            || (x, y) == Direction::Up.apply_to(level.get_player())
        {
            return vec![];
        }

        if level.get_object((x, y + 1)).placeholder() {
            return vec![Request::MoveObj((x, y), (x, y + 1))];
        }

        for side in [x - 1, x + 1] {
            if level.get_object((side, y)).placeholder()
                && level.get_object((side, y + 1)).placeholder()
            {
                return vec![Request::MoveObj((x, y), (side, y + 1))];
            }
        }

        vec![]
    }
}
