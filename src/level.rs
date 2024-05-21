use std::collections::{HashMap, HashSet};

mod objects;
pub use objects::Obj;
pub use objects::Object;
use objects::Request;

type Point = (usize, usize); // (x, y)

pub enum State {
    Win,
    Lose,
}

pub struct Update<'a> {
    pub score: usize,
    pub max_score: usize,
    pub state: Option<State>,
    pub damaged: Vec<(Point, Option<&'a Object>)>,
}

#[derive(Default)]
pub struct Level {
    score: usize,
    max_score: usize,
    state: Option<State>,
    player: Point,
    rocks: HashSet<Point>,
    damaged: HashSet<Point>,
    objects: HashMap<Point, Object>,
}

impl Level {
    pub fn parse(string: &str) -> Result<Self, String> {
        let mut level = Self::default();

        for (y, line) in string.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr != ' ' {
                    let obj = objects::parse(chr)?;
                    level.handle_request(obj.init());
                    level.add_obj((x, y), obj);
                }
            }
        }

        Ok(level)
    }

    pub fn get_update(&mut self) -> Update {
        Update {
            state: self.state.take(),
            score: self.score,
            max_score: self.max_score,
            damaged: std::mem::take(&mut self.damaged)
                .into_iter()
                .map(|point| (point, self.objects.get(&point)))
                .collect(),
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

    fn add_obj(&mut self, point: Point, obj: Object) {
        if obj.rock() {
            self.rocks.insert(point);
        } else if obj.player() {
            self.player = point;
        }

        self.damaged.insert(point);
        self.objects.insert(point, obj);
    }

    fn rem_obj(&mut self, point: &Point, handle: bool) -> Option<Object> {
        let obj = self.objects.get(point)?;
        if obj.rock() {
            self.rocks.remove(point);
        }
        if handle {
            self.handle_request(obj.on_broken());
        }

        self.damaged.insert(*point);
        self.objects.remove(point)
    }

    fn move_obj(&mut self, point: &Point, dir: &Dir) {
        let dir_point = dir.apply_to(point);
        self.rem_obj(&dir_point, true);

        if let Some(obj) = self.rem_obj(point, false) {
            self.add_obj(dir_point, obj);
        }
    }

    fn empty(&self, mut point: Point, directions: &[Dir]) -> bool {
        for dir in directions {
            point = dir.apply_to(&point);
            if self.objects.get(&point).is_some() {
                return false;
            }
        }
        true
    }

    pub fn tick(&mut self, direction: Option<Dir>) -> Update {
        let mut can_break_player = true;

        if let Some(dir) = direction {
            let next_point = dir.apply_to(&self.player);
            let next_obj = self.objects.get(&next_point);

            if let Some(next_obj) = next_obj {
                if next_obj.broken_by_player() {
                    self.move_obj(&self.player.clone(), &dir);
                    can_break_player = false;
                } else if next_obj.rock()
                    && dir != Dir::Up
                    && self.objects.get(&dir.apply_to(&next_point)).is_none()
                {
                    self.move_obj(&next_point, &dir);
                    self.move_obj(&self.player.clone(), &dir);
                }
            }
        }

        let mut rocks = Vec::from_iter(self.rocks.clone());
        rocks.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        for point in rocks {
            match self.objects.get(&Dir::Down.apply_to(&point)) {
                Some(obj) => {
                    if obj.player() && can_break_player {
                        self.move_obj(&point, &Dir::Down);
                    } else if self.empty(point, &[Dir::Left, Dir::Down]) {
                        self.move_obj(&point, &Dir::Left);
                        self.move_obj(&point, &Dir::Down);
                    } else if self.empty(point, &[Dir::Right, Dir::Down]) {
                        self.move_obj(&point, &Dir::Right);
                        self.move_obj(&point, &Dir::Down);
                    }
                }
                None => self.move_obj(&point, &Dir::Down),
            }
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
    const fn get_values(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    const fn apply_to(&self, point: &Point) -> Point {
        let (x, y) = self.get_values();
        (
            point.0.saturating_add_signed(x),
            point.1.saturating_add_signed(y),
        )
    }
}
