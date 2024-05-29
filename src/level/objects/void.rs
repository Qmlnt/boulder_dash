use super::Obj;

pub struct Void;

impl Obj for Void {
    fn char(&self) -> char {
        ' '
    }
    fn emoji(&self) -> char {
        '　'
    }
    fn name(&self) -> &str {
        "void"
    }

    fn void(&self) -> bool {
        true
    }
}
