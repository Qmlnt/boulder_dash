use super::{Behaviour, Labels, Properties};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Void;

impl Labels for Void {
    fn char(&self) -> char {
        ' '
    }
    fn emoji(&self) -> char {
        '　'
    }
}

impl Properties for Void {
    fn placeholder(&self) -> bool {
        true
    }
}

impl Behaviour for Void {}
