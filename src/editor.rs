use crate::{
    objects::{self, Object},
    Input, Mode,
    Config,
    Direction,
};
use std::{error::Error, fs};

pub type Point = (usize, usize); // (x, y)

#[derive(Default)]
pub struct Editor {
    cursor: Point,
    damaged: Vec<Point>,
    matrix: Vec<Vec<Object>>,
}

impl Editor {
    pub const fn get_cursor_pos(&self) -> &Point {
        &self.cursor
    }
    pub fn get_damaged(&mut self) -> Vec<Point> {
        std::mem::take(&mut self.damaged)
    }
    pub fn get_object(&self, (x, y): Point) -> &Object {
        &self.matrix[y][x]
    }
    pub const fn get_objects(&self) -> &Vec<Vec<Object>> {
        &self.matrix
    }
}

impl Editor {
    pub fn new(file_name: &str) -> Result<Self, String> {
        let mut editor = Editor::default();
        editor.read_file(file_name);
        Ok(editor)
    }

    fn read_file(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        for (y, line) in contents.lines().enumerate() {
            self.matrix // TODO some error callback in map() functions
                .push(line.chars().map(|c| objects::parse(c).unwrap()).collect());
            self.damaged.extend((0..line.len()).map(|x| (x, y)));
        }

        Ok(())
    }

    pub fn run (&mut self, config: &mut Config, mode: &mut Mode) {
        
    }

    fn tick(&mut self) {
        

    }
    //pub fn tick(&mut self,
}
