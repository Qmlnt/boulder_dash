use crate::{
    direction::Direction,
    interaction::{Drawable, Input, Interaction, Mode},
    objects::{Labels, Object},
    Config,
};
use std::{collections::HashSet, fs, process, thread, time::Duration};

pub type Point = (usize, usize); // (x, y)

#[derive(Default)]
pub struct Editor {
    file_name: String,
    pen_down: bool,
    current_object: usize,
    cursor: Point,
    damaged: HashSet<Point>,
    matrix: Vec<Vec<Object>>,
}

impl Editor {
    pub const fn get_cursor(&self) -> &Point {
        &self.cursor
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

impl Drawable for Editor {
    fn get_cursor(&self) -> Option<&Point> {
        Some(self.get_cursor())
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
    fn get_status(&self, _: &Config) -> String {
        let (x, y) = *self.get_cursor();
        let mut objects: Vec<String> = Object::all_objects().iter().map(|o| o.name()).collect();
        objects[self.current_object].insert(0, '<');
        objects[self.current_object].push('>');
        let pen = if self.pen_down { "down" } else { "up" };

        format!("Pen {pen}\nCursor pos: ({x} {y})\n{}", objects.join(" "))
    }
}

impl Editor {
    pub fn new(file_name: &str) -> Result<Self, String> {
        let mut editor = Self {
            file_name: file_name.to_string(),
            ..Default::default()
        };
        editor.reload()?;
        Ok(editor)
    }

    fn reload(&mut self) -> Result<(), String> {
        self.matrix = vec![];

        let contents = fs::read_to_string(&self.file_name).map_err(|e| e.to_string())?;
        for (y, line) in contents.lines().enumerate() {
            self.matrix.push(line.chars().map(Object::new).collect());
            self.damaged.extend((0..line.len()).map(|x| (x, y)));
        }

        Ok(())
    }

    fn save(&mut self) -> Result<(), String> {
        let mut contents = String::new();

        for row in &self.matrix {
            let mut line = String::new();
            for obj in row {
                line.push(obj.char());
            }
            line.push('\n');
            contents += &line;
        }

        fs::write(&self.file_name, contents).map_err(|e| e.to_string())
    }

    pub fn run(&mut self, config: &mut Config, interaction: &mut Mode) -> Result<(), String> {
        interaction.draw(self, config).map_err(|e| e.to_string())?;

        let objects = Object::all_objects();

        loop {
            thread::sleep(Duration::from_millis(25));

            let mut direction = None;

            let input = interaction.get_input();
            match input {
                Input::Q => {
                    self.save()?;
                    process::exit(0);
                }
                Input::R => self.reload()?,
                Input::Space | Input::Esc => {
                    self.pen_down = !self.pen_down;
                }
                Input::Period => {
                    self.current_object += 1;
                    if self.current_object >= objects.len() {
                        self.current_object = 0;
                    }
                }
                Input::Comma => {
                    if self.current_object == 0 {
                        self.current_object = objects.len();
                    }
                    self.current_object -= 1;
                }

                Input::Up
                | Input::Down
                | Input::Left
                | Input::Right
                | Input::W
                | Input::A
                | Input::S
                | Input::D => direction = Direction::from_input(&input),

                Input::Unknown => continue,
            }

            if let Some(dir) = direction {
                self.cursor = dir.apply_to(&self.cursor);
            }

            let (x, y) = self.cursor;
            while self.matrix.len() < y + 1 {
                self.matrix.push(vec![]);
            }
            while self.matrix[y].len() < x + 1 {
                self.matrix[y].push(Object::get_void());
                self.damaged.insert((self.matrix[y].len() - 1, y));
            }

            if self.pen_down {
                self.matrix[y][x] = objects[self.current_object].clone();
                self.damaged.insert(self.cursor);
            }

            interaction.draw(self, config).map_err(|e| e.to_string())?;
        }
    }
}
