use super::{Behaviour, Labels, Properties};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dirt;

impl Labels for Dirt {
    fn char(&self) -> char {
        '*'
    }
    fn emoji(&self) -> char {
        '🟨'
    }
}

impl Properties for Dirt {
    fn can_be_broken(&self) -> bool {
        true
    }
}

impl Behaviour for Dirt {}
