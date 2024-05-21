use super::Obj;

pub struct Rock;

impl Obj for Rock {
    fn rock(&self) -> bool {
        true
    }
    fn char(&self) -> &str {
        "r"
    }
}
