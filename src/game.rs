use crate::{
    args::Arguments,
    direction::Direction,
    interaction::{Drawable, Input, Interaction, Mode},
    objects::Object,
    Point,
};
use std::{
    error::Error,
    fs, io, thread,
    time::{Duration, Instant},
};

pub mod level;
use level::{Level, State};

#[derive(Default)]
pub struct Game {
    pause: bool,
    delay: Duration,
    level_idx: usize,
    levels: Vec<Level>,
    level_paths: Vec<String>,
}

impl Drawable for Game {
    fn get_damaged(&mut self) -> Vec<Point> {
        self.get_level_mut().get_damaged().into_iter().collect()
    }
    fn get_objects(&self) -> &Vec<Vec<Object>> {
        self.get_level().get_objects()
    }
    fn get_object(&self, (x, y): Point) -> Option<&Object> {
        self.get_level().get_objects().get(y)?.get(x)
    }

    fn get_status(&self) -> String {
        match self.get_level().get_state() {
            Some(State::Win) => "You have won!".to_string(),
            Some(State::Lose) => "You have lost!\nR - reload".to_string(),
            None => format!(
                "Score: {}/{}\nDelay: {}ms\nPaused: {}",
                self.get_level().get_score(),
                self.get_level().get_max_score(),
                self.delay.as_millis(),
                if self.pause { "yes" } else { "no" }
            ),
        }
    }
}

impl Game {
    fn get_level(&self) -> &Level {
        &self.levels[self.level_idx]
    }
    fn get_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.level_idx]
    }
    fn get_level_path(&self) -> &str {
        &self.level_paths[self.level_idx]
    }

    pub fn new(args: &Arguments) -> io::Result<Self> {
        let mut game = Self {
            pause: args.pause,
            delay: args.delay,
            level_paths: args.level_paths.clone(),
            ..Default::default()
        };

        for path in &args.level_paths {
            game.levels.push(Level::new(&fs::read_to_string(path)?));
        }

        Ok(game)
    }

    pub fn run(&mut self, interaction: &mut Mode) -> Result<(), Box<dyn Error>> {
        let mut direction = None;
        let mut paused_on_start = true;
        let mut timer = Instant::now();

        interaction.draw(self)?;

        loop {
            thread::sleep(Duration::from_millis(10));

            let input = interaction.get_input();
            match input {
                Input::Quit | Input::Q => return Ok(()),
                Input::Comma => {
                    if self.delay.as_millis() >= 100 {
                        self.delay -= Duration::from_millis(50);
                    }
                }
                Input::Period => {
                    if self.delay.as_millis() <= 950 {
                        self.delay += Duration::from_millis(50);
                    }
                }
                Input::Esc | Input::Space => self.pause = !self.pause,
                Input::R => {
                    self.levels[self.level_idx] =
                        Level::new(&fs::read_to_string(self.get_level_path())?);
                    direction = None;
                    paused_on_start = true;
                    interaction.draw(self)?;
                    continue;
                }

                Input::Up
                | Input::Down
                | Input::Left
                | Input::Right
                | Input::W
                | Input::A
                | Input::S
                | Input::D => direction = Direction::try_from(input.clone()).ok(),

                Input::Unknown => (),
            }

            if let Some(state) = self.get_level().get_state() {
                if *state == State::Win && self.level_idx + 1 < self.levels.len() {
                    self.level_idx += 1;
                    interaction.draw(self)?;
                }
                continue;
            }

            if timer.elapsed() < self.delay {
                if input != Input::Unknown {
                    interaction.draw(self)?;
                }
                continue;
            }

            timer = Instant::now();

            if paused_on_start && direction.is_some() {
                paused_on_start = false;
            }
            if (self.pause && direction.is_none()) || paused_on_start {
                continue;
            }

            self.get_level_mut().tick(direction.take());
            interaction.draw(self)?;
        }
    }
}
