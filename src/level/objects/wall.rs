use super::{Behaviour, Labels, Properties};

#[derive(Debug)]
pub struct Wall;

impl Labels for Wall {
    fn char(&self) -> char {
        '#'
    }
    fn emoji(&self) -> char {
        '🧱'
    }
}

impl Properties for Wall {}
impl Behaviour for Wall {}
