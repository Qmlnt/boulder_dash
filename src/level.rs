mod direction;
mod request;
mod traits;

use crate::objects::Object;
pub use {
    direction::Direction,
    request::Request,
    traits::{Behaviour, Properties},
};

pub type Point = (usize, usize); // (x, y)

#[derive(Clone, PartialEq, Eq)]
pub enum State {
    Win,
    Lose,
}

#[derive(Default)]
pub struct Level {
    score: usize,
    max_score: usize,
    state: Option<State>,
    player: Point,
    damaged: Vec<Point>,
    matrix: Vec<Vec<Object>>,
}

// Getters
impl Level {
    pub const fn get_score(&self) -> &usize {
        &self.score
    }
    pub const fn get_max_score(&self) -> &usize {
        &self.max_score
    }
    pub const fn get_player_pos(&self) -> &Point {
        &self.player
    }
    pub fn get_damaged(&mut self) -> Vec<Point> {
        std::mem::take(&mut self.damaged)
    }
    pub const fn get_state(&self) -> &Option<State> {
        &self.state
    }
    pub fn get_object(&self, (x, y): Point) -> &Object {
        &self.matrix[y][x]
    }
    pub const fn get_objects(&self) -> &Vec<Vec<Object>> {
        &self.matrix
    }
}

impl Level {
    pub fn parse(string: &str) -> Result<Self, String> {
        let mut level = Self::default();

        for (y, line) in string.lines().enumerate() {
            let mut row = vec![];

            for (x, chr) in line.chars().enumerate() {
                let obj = crate::objects::parse(chr)?;
                level.handle_requests(obj.init());
                if obj.player() {
                    level.player = (x, y);
                }

                level.damaged.push((x, y));
                row.push(obj);
            }
            level.matrix.push(row);
        }

        Ok(level)
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
                Request::MoveObj(from, to) => {
                    if self.get_object(from).player() {
                        self.player = to;
                    }

                    self.move_obj(from, to);
                    self.damaged.extend([from, to]);
                }
            }
        }
    }

    fn move_obj(&mut self, from: Point, to: Point) {
        self.matrix[to.1][to.0] = std::mem::replace(
            &mut self.matrix[from.1][from.0],
            crate::objects::get_placeholder(),
        );
    }

    pub fn tick(&mut self, direction: Option<Direction>) {
        // Player
        let requests = self
            .get_object(self.player)
            .tick(self, self.player, direction);
        self.handle_requests(requests);

        // Rocks
        for y in (0..self.matrix.len()).rev() {
            for x in 0..self.matrix[y].len() {
                if self.get_object((x, y)).can_be_moved() {
                    self.handle_requests(self.get_object((x, y)).tick(self, (x, y), None));
                }
            }
        }
    }
}
