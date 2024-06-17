use super::{Behaviour, Direction, Labels, Level, Point, Properties, Request, State};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player;

impl Labels for Player {
    fn char(&self) -> char {
        'p'
    }
    fn emoji(&self) -> char {
        '🦀'
    }
}

impl Properties for Player {
    fn player(&self) -> bool {
        true
    }
}

impl Behaviour for Player {
    fn tick(&self, level: &Level, _: Point, direction: Option<Direction>) -> Vec<Request> {
        let mut requests = vec![];

        // to prevent the rock from falling on player when object underneath is broken
        let mut player_broke = false;
        let mut above_point = Direction::Up.apply_to(level.get_player());

        // Player
        if let Some(dir) = direction {
            let next_point = dir.apply_to(level.get_player());

            player_broke = level.get_object(next_point).can_be_broken();
            let can_move_next = matches!(dir, Direction::Left | Direction::Right)
                && level.get_object(next_point).can_be_moved()
                && level.get_object(dir.apply_to(&next_point)).placeholder();

            if player_broke {
                requests.extend(level.get_object(next_point).on_broken(level));
            } else if can_move_next {
                requests.push(Request::MoveObj {
                    from: next_point,
                    to: dir.apply_to(&next_point),
                });
            }

            if level.get_object(next_point).placeholder() || player_broke || can_move_next {
                requests.push(Request::MoveObj {
                    from: *level.get_player(),
                    to: next_point,
                });
                above_point = Direction::Up.apply_to(&next_point);
            }
        }

        // Check the rock above
        if !player_broke && level.get_object(above_point).can_be_moved() {
            requests.push(Request::UpdateState(State::Lose));
            requests.push(Request::MoveObj {
                from: above_point,
                to: Direction::Down.apply_to(&above_point),
            });
        }

        requests
    }
}
