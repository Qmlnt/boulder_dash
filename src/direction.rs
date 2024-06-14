use crate::interaction::Input;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<Input> for Direction {
    type Error = ();

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        match input {
            Input::Up | Input::W => Ok(Self::Up),
            Input::Down | Input::S => Ok(Self::Down),
            Input::Left | Input::A => Ok(Self::Left),
            Input::Right | Input::D => Ok(Self::Right),

            _ => Err(()),
        }
    }
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
