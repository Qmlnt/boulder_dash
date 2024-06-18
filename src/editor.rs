use crate::{
    args::Arguments,
    direction::Direction,
    interaction::{Drawable, Input, Interaction, Mode},
    objects::{Labels, Object},
    Point,
};
use std::{collections::HashSet, error::Error, fs, io, thread, time::Duration};

#[derive(Default)]
pub struct Editor {
    file_name: String,
    cursor: Point,
    pen_down: bool,
    current_object: usize,
    damaged: HashSet<Point>,
    matrix: Vec<Vec<Object>>,
}

impl Drawable for Editor {
    fn get_cursor(&self) -> Option<&Point> {
        Some(&self.cursor)
    }

    fn get_damaged(&mut self) -> Vec<Point> {
        std::mem::take(&mut self.damaged).into_iter().collect()
    }
    fn get_objects(&self) -> &Vec<Vec<Object>> {
        &self.matrix
    }
    fn get_object(&self, (x, y): Point) -> Option<&Object> {
        self.matrix.get(y)?.get(x)
    }

    fn get_status(&self) -> String {
        let (x, y) = self.cursor;
        let mut objects: Vec<String> = Object::get_all_displayable()
            .iter()
            .map(Labels::name)
            .collect();
        objects[self.current_object].insert(0, '[');
        objects[self.current_object].push(']');
        let pen = if self.pen_down { "down" } else { "up" };

        format!("Pen {pen}\nCursor pos: ({x}, {y})\n{}", objects.join(" "))
    }
}

impl Editor {
    pub fn new(args: &Arguments) -> io::Result<Self> {
        let mut editor = Self {
            file_name: args.level_paths[0].clone(),
            ..Default::default()
        };
        editor.reload()?;
        Ok(editor)
    }

    fn reload(&mut self) -> io::Result<()> {
        self.matrix = vec![];

        let contents = fs::read_to_string(&self.file_name)?;
        for (y, line) in contents.trim().lines().map(str::trim).enumerate() {
            self.matrix.push(line.chars().map(Object::new).collect());
            self.damaged.extend((0..line.len()).map(|x| (x, y)));
        }

        if self.matrix.is_empty() {
            self.matrix.push(vec![Object::default()]);
            self.damaged.insert((0, 0));
        }

        Ok(())
    }

    fn save(&mut self) -> io::Result<()> {
        let mut contents = String::new();

        for row in &self.matrix {
            contents += row.iter().map(Labels::char).collect::<String>().trim();
            contents.push('\n');
        }

        fs::write(&self.file_name, contents.trim())
    }

    pub fn run(&mut self, interaction: &mut Mode) -> Result<(), Box<dyn Error>> {
        interaction.draw(self)?;

        let objects = Object::get_all_displayable();

        loop {
            thread::sleep(Duration::from_millis(25));

            let mut direction = None;

            let input = interaction.get_input();
            match input {
                Input::Quit | Input::Q => {
                    self.save()?;
                    return Ok(());
                }
                Input::R => {
                    self.reload()?;
                    self.pen_down = false;
                }
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
                match dir {
                    Direction::Up => {
                        if self.matrix.len() > 1
                            && self
                                .matrix
                                .last()
                                .map_or(false, |l| l.iter().all(|o| *o == Object::default()))
                        {
                            self.matrix.pop();
                        }
                    }
                    Direction::Left => {
                        for row in &mut self.matrix {
                            while self.cursor.0 < row.len()
                                && *row.last().unwrap() == Object::default()
                            {
                                row.pop();
                            }
                        }
                    }
                    _ => (),
                }

                self.damaged.insert(self.cursor);
                self.cursor = dir.apply_to(&self.cursor);
            }

            let (x, y) = self.cursor;

            while y + 1 > self.matrix.len() {
                self.matrix.push(vec![]);
            }
            // Moving up or down on a shorter row
            while x + 1 > self.matrix[y].len() {
                self.matrix[y].push(Object::default());
                self.damaged.insert((self.matrix[y].len() - 1, y));
            }

            if self.pen_down {
                self.matrix[self.cursor.1][self.cursor.0] = objects[self.current_object].clone();
                self.damaged.insert(self.cursor);
            }

            interaction.draw(self)?;
        }
    }
}
