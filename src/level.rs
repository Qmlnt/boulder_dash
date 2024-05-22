mod objects;
pub use objects::Obj;
pub use objects::Object;
use objects::Request;

type Point = (usize, usize); // (x, y)

#[derive(Clone)]
pub enum State {
    Win,
    Lose,
}

pub struct Update<'a> {
    pub score: usize,
    pub max_score: usize,
    pub state: Option<State>,
    pub matrix: &'a Vec<Vec<Object>>,
}

#[derive(Default)]
pub struct Level {
    score: usize,
    max_score: usize,
    state: Option<State>,
    player: Point,
    matrix: Vec<Vec<Object>>,
}

impl Level {
    pub fn parse(string: &str) -> Result<Self, String> {
        let mut level = Self::default();

        for line in string.lines() {
            let mut row = vec![];

            for chr in line.chars() {
                let obj = objects::parse(chr)?;
                level.handle_request(obj.init());
                if obj.player() {
                    level.player = (row.len(), level.matrix.len());
                }

                row.push(obj);
            }
            level.matrix.push(row);
        }

        Ok(level)
    }

    pub fn get_update(&self) -> Update {
        Update {
            score: self.score,
            max_score: self.max_score,
            state: self.state.clone(),
            matrix: &self.matrix,
        }
    }

    fn handle_request(&mut self, request: Option<Request>) {
        if let Some(request) = request {
            match request {
                Request::AddScore => self.score += 1,
                Request::AddMaxScore => self.max_score += 1,
                Request::GameLost => self.state = Some(State::Lose),
            }
        }
    }

    fn get_obj(&self, (x, y): Point) -> &Object {
        &self.matrix[y][x]
    }

    fn move_obj(&mut self, from: Point, to: Point) {
        self.matrix[to.1][to.0] = std::mem::replace(
            &mut self.matrix[from.1][from.0],
            objects::parse(' ').expect("new Void object"),
        );
    }

    pub fn tick(&mut self, direction: Option<Dir>) {
        // to prevent the rock from falling on player when object underneath is broken
        let mut player_broke = false;

        // Player
        if let Some(dir) = direction {
            let next_point = dir.apply_to(&self.player);

            player_broke = self.get_obj(next_point).breakable();
            let move_next = matches!(dir, Dir::Left | Dir::Right)
                && self.get_obj(next_point).rock()
                && self.get_obj(dir.apply_to(&next_point)).void();

            if player_broke {
                self.handle_request(self.get_obj(next_point).on_broken());
            } else if move_next {
                self.move_obj(next_point, dir.apply_to(&next_point));
            }

            if self.get_obj(next_point).void() || player_broke || move_next {
                self.move_obj(self.player, next_point);
                self.player = next_point;
            }
        }

        // Check the rock above
        let above_point = Dir::Up.apply_to(&self.player);
        if !player_broke && self.get_obj(above_point).rock() {
            self.state = Some(State::Lose);
            self.move_obj(above_point, self.player);
        }

        // Rocks
        for y in (0..self.matrix.len()).rev() {
            for x in 0..self.matrix[y].len() {
                if !self.get_obj((x, y)).rock() || (x, y) == self.player || (x, y) == above_point {
                    continue;
                }

                if self.get_obj((x, y + 1)).void() {
                    self.move_obj((x, y), (x, y + 1));
                } else {
                    for side in [x - 1, x + 1] {
                        if self.get_obj((side, y)).void() && self.get_obj((side, y + 1)).void() {
                            self.move_obj((x, y), (side, y + 1));
                            break;
                        }
                    }
                }
            }
        }

        if self.score == self.max_score {
            self.state = Some(State::Win);
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    const fn apply_to(&self, point: &Point) -> Point {
        let (x, y) = match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        };

        (
            point.0.saturating_add_signed(x),
            point.1.saturating_add_signed(y),
        )
    }
}
