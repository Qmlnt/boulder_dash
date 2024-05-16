use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

mod objects;
use objects::{LevelObj, ObjEvent};

type Point = (usize, usize); // (x, y)

pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

impl Move {
    const fn get_values(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Stay => (0, 0),
        }
    }

    const fn apply_to(&self, point: Point) -> Point {
        let (x, y) = self.get_values();
        (
            point.0.saturating_add_signed(x),
            point.1.saturating_add_signed(y),
        )
    }
}

pub enum LevelState<'a> {
    Win,
    Lose,
    Idle,
    Init(Level<'a>),
    Update {
        score: usize,
        damaged: HashMap<Point, Option<&'a LevelObj>>,
    },
}
//
// pub enum LevelUpdate<'a> {
//     Score(usize),
//     MaxScore(usize),
//     State(LevelState),
//     Hook(&'a dyn LevelObj),
//     Damaged(HashMap<Point, Option<&'a dyn LevelObj>>),
// }
//
#[derive(Default)]
pub struct Level<'a> {
    score: usize,
    max_score: usize,
    hooks: Vec<&'a LevelObj>,
    objects: HashMap<Point, LevelObj>,
}

impl Level<'_> {
    pub fn parse(string: &str) -> Result<LevelState, String> {
        let mut level = Self::default();

        for (y, line) in string.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == ' ' {
                    continue;
                }
                let obj = LevelObj::parse(chr)?;

                level.objects.insert((x, y), obj);

                let obj_props = obj.get_props();

                if obj_props.fall == true || obj_props.control == true {
                    level.hooks.push(&obj);
                }
                if let Some(ObjEvent::AddScore(num)) = obj_props.on_broken {
                    level.max_score += num as usize;
                }
            }
        }

        Ok(LevelState::Init(level))
    }

    pub fn tick(&mut self, mv: Move) -> LevelState {
        let mut damaged = HashMap::new();
        for obj in &self.hooks {
            // if !damaged.contains(obj) {
            //     self.objects[obj].trigger_hook(self);
            // }
        }

        // let mut damaged = HashMap::<Point, Option<Box<dyn LevelObj>>>::new();
        // for point in std::mem::take(&mut self.damaged) {
        // if let Some(obj) = self.objects.get(&point) {
        //     damaged.insert(point, obj.clone());
        // } else {
        //     damaged.insert(point, None);
        // }
        // }

        LevelState::Update {
            damaged,
            score: self.score,
        }
    }
}
