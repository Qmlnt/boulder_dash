use crate::{
    args::Config,
    direction::Direction,
    interaction::{Drawable, Input, Interaction, Mode},
    objects::{Behaviour, Object, Properties},
    Point,
};
use std::{
    collections::HashSet,
    fs, process, thread,
    time::{Duration, Instant},
};

#[derive(Clone, PartialEq, Eq)]
pub enum State {
    Win,
    Lose,
}

pub enum Request {
    AddScore,
    AddMaxScore,
    UpdateState(State),
    MoveObj(Point, Point), // (from, to)
}

#[derive(Default)]
pub struct Level {
    file_name: String,
    score: usize,
    max_score: usize,
    state: Option<State>,
    player: Point,
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

impl Drawable for Level {
    fn get_cursor(&self) -> &Point {
        self.get_player()
    }
    fn get_damaged(&mut self) -> HashSet<Point> {
        self.get_damaged()
    }
    fn get_objects(&self) -> &Vec<Vec<Object>> {
        self.get_objects()
    }
    fn get_object(&self, point: Point) -> &Object {
        self.get_object(point)
    }
    fn get_status(&self, config: &Config) -> String {
        match self.state {
            Some(State::Win) => "You have won!".to_string(),
            Some(State::Lose) => "You have lost!".to_string(),
            None => format!(
                "Score: {}/{}\nDelay: {}ms\nPaused: {}",
                self.get_score(),
                self.get_max_score(),
                config.delay.as_millis(),
                if config.pause { "yes" } else { "no" }
            ),
        }
    }
}

impl Level {
    pub fn new(file_name: &str) -> Result<Self, String> {
        let mut level = Self {
            file_name: file_name.to_string(),
            ..Default::default()
        };
        level.reload()?;
        Ok(level)
    }

    fn reload(&mut self) -> Result<(), String> {
        self.max_score = 0;
        self.matrix = vec![];

        let contents = fs::read_to_string(&self.file_name).map_err(|e| e.to_string())?;
        for (y, line) in contents.lines().enumerate() {
            let mut row = vec![];

            for (x, chr) in line.chars().enumerate() {
                let obj = Object::new(chr);
                self.handle_requests(obj.init());
                if obj.player() {
                    self.player = (x, y);
                }

                self.damaged.insert((x, y));
                row.push(obj);
            }
            self.matrix.push(row);
        }

        Ok(())
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

                    self.matrix[to.1][to.0] =
                        std::mem::replace(&mut self.matrix[from.1][from.0], Object::get_void());
                    self.damaged.extend([from, to]);
                }
            }
        }
    }

    fn tick_objects(&mut self, direction: Option<Direction>) {
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

    pub fn run(&mut self, config: &mut Config, interaction: &mut Mode) -> Result<State, String> {
        let mut launch_pause = true;
        let mut direction = None;
        let mut timer = Instant::now();

        interaction.draw(self, config)?;

        loop {
            let mut input = true;
            match interaction.get_input() {
                Input::Unknown => input = false,
                Input::Q => process::exit(0),
                Input::R => {
                    launch_pause = true;
                    self.reload()?;
                }
                Input::Comma => {
                    if config.delay.as_millis() >= 100 {
                        config.delay -= Duration::from_millis(50);
                    }
                }
                Input::Period => {
                    if config.delay.as_millis() <= 950 {
                        config.delay += Duration::from_millis(50);
                    }
                }
                Input::Esc | Input::Space => config.pause = !config.pause,

                Input::Up | Input::W => direction = Some(Direction::Up),
                Input::Down | Input::S => direction = Some(Direction::Down),
                Input::Left | Input::A => direction = Some(Direction::Left),
                Input::Right | Input::D => direction = Some(Direction::Right),
            }

            if timer.elapsed() > config.delay {
                timer = Instant::now();

                if launch_pause && direction.is_some() {
                    launch_pause = false;
                }
                if (config.pause && direction.is_none()) || launch_pause {
                    continue;
                }

                self.tick_objects(direction.take());
                interaction.draw(self, config)?;

                if let Some(state) = self.get_state() {
                    return Ok(state.clone());
                }
            } else if input {
                interaction.draw(self, config)?;
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
