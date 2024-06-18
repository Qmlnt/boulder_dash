use crate::{
    direction::Direction,
    objects::{Behaviour, Object, Properties},
    Point,
};
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq)]
pub enum State {
    Win,
    Lose,
}

pub enum Request {
    AddScore,
    AddMaxScore,
    UpdateState(State),
    MoveObj { from: Point, to: Point }, // (from, to)
}

#[derive(Default)]
pub struct Level {
    score: usize,
    max_score: usize,
    player: Point,
    state: Option<State>,
    damaged: HashSet<Point>,
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
    pub const fn get_state(&self) -> &Option<State> {
        &self.state
    }
    pub const fn get_player(&self) -> &Point {
        &self.player
    }
    pub fn get_damaged(&mut self) -> HashSet<Point> {
        std::mem::take(&mut self.damaged)
    }
    pub fn get_object(&self, (x, y): Point) -> &Object {
        &self.matrix[y][x]
    }
    pub const fn get_objects(&self) -> &Vec<Vec<Object>> {
        &self.matrix
    }
}

impl Level {
    pub fn new(string: &str) -> Self {
        let mut level = Self::default();
        for (y, line) in string.trim().lines().enumerate() {
            let mut row = vec![];

            for (x, chr) in line.trim().chars().enumerate() {
                let obj = Object::new(chr);
                level.handle_requests(obj.init());
                if obj.player() {
                    level.player = (x, y);
                }

                level.damaged.insert((x, y));
                row.push(obj);
            }
            level.matrix.push(row);
        }

        level
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
                Request::MoveObj { from, to } => {
                    if self.get_object(from).player() {
                        self.player = to;
                    }

                    self.matrix[to.1][to.0] = std::mem::take(&mut self.matrix[from.1][from.0]);
                    self.damaged.extend([from, to]);
                }
            }
        }
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
                if self.matrix[y][x].can_be_moved() {
                    self.handle_requests(self.matrix[y][x].tick(self, (x, y), None));
                }
            }
        }
    }
}
