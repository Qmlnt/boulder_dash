use std::{str::FromStr, time::Duration};

const HELP_MSG: &str = "\
FLAGS:
    -h, --help
        Show this message.
    -p, --pause
        Launch paused.
OPTIONS:
    -l, --level <string>
        Required.
        Specify a level to run.
        Can be used multiple times.
    -m, --mode <string>
        * gui
        * tui (default)
        * cli
        Select the interaction mode.
    -r, --run <string>
        * g / b / game (default)
        * e / editor
        Select the program mode.
    -s, --size <integer>
        Object size for GUI. (default 30 pixels).
    -d, --delay <integer>
        Delay between frames. (default: 1000 ms)\
";

#[derive(Debug, PartialEq, Eq)]
pub enum InteractionMode {
    Gui,
    Tui,
    Cli,
}

impl FromStr for InteractionMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gui" => Ok(Self::Gui),
            "tui" => Ok(Self::Tui),
            "cli" => Ok(Self::Cli),
            _ => Err(format!("Can't parse `{s}` as a valid display mode!")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramMode {
    Game,
    Editor,
}

impl FromStr for ProgramMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "g" | "b" | "game" => Ok(Self::Game),
            "e" | "editor" => Ok(Self::Editor),
            _ => Err(format!("Can't parse `{s}` as a valid program mode!")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arguments {
    pub size: u32,
    pub pause: bool,
    pub delay: Duration,
    pub level_paths: Vec<String>,
    pub program_mode: ProgramMode,
    pub interaction_mode: InteractionMode,
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            size: 30,
            pause: false,
            delay: Duration::from_millis(1000),
            level_paths: vec![],
            program_mode: ProgramMode::Game,
            interaction_mode: InteractionMode::Tui,
        }
    }
}

fn parse_arg<T, E>(arg_opt: Option<String>, arg_name: &str) -> Result<T, String>
where
    T: FromStr<Err = E>,
    E: ToString,
{
    match arg_opt {
        Some(arg) => arg.parse().map_err(|e: E| e.to_string()),
        None => Err(format!("Missing value for `{arg_name}`!")),
    }
}

impl Arguments {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut config = Self::default();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    println!("{HELP_MSG}");
                    std::process::exit(0);
                }
                "-p" | "--pause" => config.pause = true,

                "-s" | "--size" => config.size = parse_arg(args.next(), arg.as_str())?,
                "-d" | "--delay" => {
                    config.delay = Duration::from_millis(parse_arg(args.next(), arg.as_str())?);
                }
                "-l" | "--level" => config
                    .level_paths
                    .push(parse_arg(args.next(), arg.as_str())?),
                "-r" | "--run" => config.program_mode = parse_arg(args.next(), arg.as_str())?,
                "-m" | "--mode" => config.interaction_mode = parse_arg(args.next(), arg.as_str())?,

                _ => return Err(format!("Unrecognized option `{arg}`!")),
            }
        }

        match config.level_paths.first() {
            Some(_) => Ok(config),
            None => Err("Specify a level path with `-l some/path`!".into()),
        }
    }
}
