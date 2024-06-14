use super::{Behaviour, Labels, Properties};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unknown;

impl Labels for Unknown {
    fn char(&self) -> char {
        '?'
    }
    fn emoji(&self) -> char {
        '🯄'
    }
}

impl Properties for Unknown {
    fn placeholder(&self) -> bool {
        true
    }
}

impl Behaviour for Unknown {}
