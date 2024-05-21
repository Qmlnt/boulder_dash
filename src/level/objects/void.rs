use super::Obj;

pub struct Void;

impl Obj for Void {
    fn void(&self) -> bool {
        true
    }
    fn char(&self) -> &str {
        "　"
    }
}
