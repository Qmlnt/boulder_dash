#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn apply_to(&self, point: &(usize, usize)) -> (usize, usize) {
        let (x, y) = match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        };

        (
            point.0.saturating_add_signed(x),
            point.1.saturating_add_signed(y),
        )
    }
}
