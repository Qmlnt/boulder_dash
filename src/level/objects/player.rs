use super::{Dir, Level, Obj, Point, Request, State};

pub struct Player;

impl Obj for Player {
    fn char(&self) -> char {
        'p'
    }
    fn emoji(&self) -> char {
        '🦀'
    }
    fn name(&self) -> &str {
        "player"
    }

    fn player(&self) -> bool {
        true
    }

    fn tick(&self, level: &Level, _: Point, direction: Option<Dir>) -> Vec<Request> {
        let mut requests = vec![];

        // to prevent the rock from falling on player when object underneath is broken
        let mut player_broke = false;
        let mut above_point = Dir::Up.apply_to(&level.player);

        // Player
        if let Some(dir) = direction {
            let next_point = dir.apply_to(&level.player);

            player_broke = level.get_obj(next_point).breakable();
            let can_move_next = matches!(dir, Dir::Left | Dir::Right)
                && level.get_obj(next_point).rock()
                && level.get_obj(dir.apply_to(&next_point)).void();

            if player_broke {
                requests.extend(level.get_obj(next_point).on_broken(level));
            } else if can_move_next {
                requests.push(Request::MoveObj(next_point, dir.apply_to(&next_point)));
            }

            if level.get_obj(next_point).void() || player_broke || can_move_next {
                requests.push(Request::MoveObj(level.player, next_point));
                above_point = Dir::Up.apply_to(&next_point);
            }
        }

        // Check the rock above
        if !player_broke && level.get_obj(above_point).rock() {
            requests.push(Request::UpdateState(State::Lose));
            requests.push(Request::MoveObj(above_point, level.player));
        }

        requests
    }
}
