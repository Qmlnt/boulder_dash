use crate::{
    direction::Direction,
    interaction::{Drawable, Input, Interaction, Mode},
    objects::{Labels, Object},
    Point,
};
use std::{collections::HashSet, error::Error, fs, io, process, thread, time::Duration};

#[derive(Default)]
pub struct Editor {
    file_name: String,
    cursor: Point,
    pen_down: bool,
    current_object: usize,
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
    fn get_damaged(&mut self) -> Vec<Point> {
        self.get_damaged().into_iter().collect()
    }
    fn get_objects(&self) -> &Vec<Vec<Object>> {
        self.get_objects()
    }
    fn get_object(&self, point: Point) -> &Object {
        self.get_object(point)
    }
    fn get_status(&self) -> String {
        let (x, y) = *self.get_cursor();
        let mut objects: Vec<String> = Object::get_all_valid().iter().map(Labels::name).collect();
        objects[self.current_object].insert(0, '[');
        objects[self.current_object].push(']');
        let pen = if self.pen_down { "down" } else { "up" };

        format!("Pen {pen}\nCursor pos: ({x}, {y})\n{}", objects.join(" "))
    }
}

impl Editor {
    pub fn new(file_name: &str) -> io::Result<Self> {
        let mut editor = Self {
            file_name: file_name.to_owned(),
            ..Default::default()
        };
        editor.reload()?;
        Ok(editor)
    }

    fn reload(&mut self) -> io::Result<()> {
        self.matrix = vec![];

        let contents = fs::read_to_string(&self.file_name)?;
        for (y, line) in contents.trim().lines().enumerate() {
            self.matrix.push(line.chars().map(Object::new).collect());
            self.damaged.extend((0..line.len()).map(|x| (x, y)));
        }

        if self.matrix.is_empty() {
            self.matrix.push(vec![Object::get_void()]);
            self.damaged.insert((0, 0));
        }

        Ok(())
    }

    fn save(&mut self) -> io::Result<()> {
        let mut contents = String::new();

        for row in &self.matrix {
            let mut line = String::new();
            for obj in row {
                line.push(obj.char());
            }
            contents += line.trim();
            contents.push('\n');
        }

        fs::write(&self.file_name, contents.trim())
    }

    pub fn run(&mut self, interaction: &mut Mode) -> Result<(), Box<dyn Error>> {
        interaction.draw(self)?;

        let objects = Object::get_all_valid();

        loop {
            thread::sleep(Duration::from_millis(25));

            let mut direction = None;

            let input = interaction.get_input();
            match input {
                Input::Quit | Input::Q => {
                    self.save()?;
                    process::exit(0);
                }
                Input::R => self.reload()?,
                Input::Esc => self.save()?,
                Input::Space => {
                    self.pen_down = !self.pen_down;
                }
                Input::Comma => {
                    if self.current_object == 0 {
                        self.current_object = objects.len();
                    }
                    self.current_object -= 1;
                }
                Input::Period => {
                    self.current_object += 1;
                    if self.current_object >= objects.len() {
                        self.current_object = 0;
                    }
                }

                Input::Up
                | Input::Down
                | Input::Left
                | Input::Right
                | Input::W
                | Input::A
                | Input::S
                | Input::D => direction = Direction::try_from(input).ok(),

                Input::Unknown => continue,
            }

            if let Some(dir) = direction {
                self.damaged.insert(self.cursor);
                self.cursor = dir.apply_to(&self.cursor);
            }

            let (x, y) = self.cursor;
            while y + 1 > self.matrix.len() {
                self.matrix.push(vec![]);
            }
            // Moving up or down on a shorter row
            while x + 1 > self.matrix[y].len() {
                self.matrix[y].push(Object::get_void());
                self.damaged.insert((self.matrix[y].len() - 1, y));
            }

            if self.pen_down {
                self.matrix[y][x] = objects[self.current_object].clone();
                self.damaged.insert(self.cursor);
            }

            interaction.draw(self)?;
        }
    }
}
