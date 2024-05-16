pub struct Props {
    pub fall: bool,
    pub control: bool,
    pub move_force: u8,
    pub break_force: u8,
    pub moved_with: u8,
    pub broken_with: u8,
    pub on_broken: Option<ObjEvent>,
}

pub enum ObjEvent {
    AddScore(u8),
    GameOver,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            fall: false,
            control: false,
            move_force: 0,
            break_force: 0,
            moved_with: u8::MAX,
            broken_with: u8::MAX,
            on_broken: None,
        }
    }
}

pub enum LevelObj {
    Gem,
    Wall,
    Dirt,
    Rock,
    Player,
}

impl LevelObj {
    pub fn parse(chr: char) -> Result<Self, String> {
        Ok(match chr {
            'g' => Self::Gem,
            '#' => Self::Wall,
            'd' => Self::Dirt,
            'r' => Self::Rock,
            'p' => Self::Player,
            _ => return Err(format!("Can't parse char `{chr}`")),
        })
    }

    pub fn get_props(&self) -> Props {
        let mut props = Props::default();

        match self {
            Self::Dirt => props.broken_with = 2,
            Self::Gem => {
                props.broken_with = 2;
                props.on_broken = Some(ObjEvent::AddScore(1));
            }
            Self::Rock => {
                props.fall = true;
                props.moved_with = 1;
                props.break_force = 1;
            }
            Self::Player => {
                props.control = true;
                props.move_force = 1;
                props.break_force = 2;
                props.broken_with = 1;
                props.on_broken = Some(ObjEvent::GameOver);
            }
            _ => (),
        }

        props
    }
}
