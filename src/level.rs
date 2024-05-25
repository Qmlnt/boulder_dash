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
                level.handle_requests(obj.init());
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

    fn handle_requests(&mut self, requests: Vec<Request>) {
        for request in requests {
            match request {
                Request::UpdateState(state) => {
                    if self.state.is_none() {
                        self.state = Some(state);
                    }
                }

                Request::AddScore => self.score += 1,
                Request::AddMaxScore => self.max_score += 1,
                Request::MoveObj(from, to) => self.move_obj(from, to),
            }
        }
    }

    fn get_obj(&self, (x, y): Point) -> &Object {
        &self.matrix[y][x]
    }

    fn move_obj(&mut self, from: Point, to: Point) {
        if self.get_obj(from).player() {
            self.player = to;
        }

        self.matrix[to.1][to.0] = std::mem::replace(
            &mut self.matrix[from.1][from.0],
            objects::parse(' ').expect("new Void object"),
        );
    }

    pub fn tick(&mut self, direction: Option<Dir>) {
        // Player
        self.handle_requests(self.get_obj(self.player).tick(self, self.player, direction));

        // Rocks
        for y in (0..self.matrix.len()).rev() {
            for x in 0..self.matrix[y].len() {
                if self.get_obj((x, y)).rock() {
                    self.handle_requests(self.get_obj((x, y)).tick(self, (x, y), None));
                }
            }
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
