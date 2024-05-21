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
                    level.player = (row.len() - 1, level.matrix.len());
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

    pub fn tick(&mut self, direction: Option<Dir>) -> Update {
        // to prevent the rock from falling on player when object underneath is broken
        let mut player_broke = match self.state {
            None => false,
            Some(State::Win) => true,
            Some(State::Lose) => return self.get_update(),
        };

        // Player
        if let Some(dir) = direction {
            let (nx, ny) = dir.apply_to(&self.player);
            let (mx, my) = dir.apply_to(&(nx, ny));
            let next_obj = self.matrix[ny][nx];

            let move_next = matches!(dir, Dir::Left | Dir::Right)
                && next_obj.rock()
                && self.matrix[my][mx].void();

            if next_obj.breakable() {
                player_broke = true;
                self.handle_request(next_obj.on_broken());
            } else if move_next {
                std::mem::swap(&mut self.matrix[ny][nx], &mut self.matrix[my][mx]);
            }

            if next_obj.void() || player_broke || move_next {
                std::mem::swap(
                    &mut self.matrix[ny][nx],
                    &mut self.matrix[self.player.1][self.player.0],
                );
                self.player = (nx, ny);
            }
        }

        // Rocks
        for row in (0..self.matrix.len()).rev() {
            for col in 0..self.matrix[row].len() {
                // Down
                if self.matrix[row + 1][col].void() {
                    std::mem::swap(&mut self.matrix[row][col], &mut self.matrix[row + 1][col]);
                } else if self.matrix[row + 1][col].player() && !player_broke {
                    self.handle_request(self.matrix[row + 1][col].on_broken());
                    std::mem::swap(&mut self.matrix[row][col], &mut self.matrix[row + 1][col]);
                }
                // Sideways
                else {
                    for side in [col - 1, col + 1] {
                        if !(self.matrix[row][side].void() && self.matrix[row + 1][side].void()) {
                            continue;
                        }
                        std::mem::swap(&mut self.matrix[row][col], &mut self.matrix[row][side]);
                    }
                }
            }
        }

        if self.score == self.max_score {
            self.state = Some(State::Win);
        }

        self.get_update()
    }
}

#[derive(PartialEq)]
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
