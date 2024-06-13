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
        Select display mode.
    -r, --run <string>
        * g / b / game (default)
        * e / editor
        Select program to run.
    -s, --size <integer>
        Object size for GUI. (default 30 pixels).
    -d, --delay <integer>
        Delay between frames. (default: 1000 ms)\
";

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    Gui,
    Tui,
    Cli,
}

impl FromStr for AppMode {
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
pub struct Config {
    pub pause: bool,
    pub size: u16,
    pub delay: Duration,
    pub app_mode: AppMode,
    pub program_mode: ProgramMode,
    pub level_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pause: false,
            size: 30,
            delay: Duration::from_millis(1000),
            program_mode: ProgramMode::Game,
            app_mode: AppMode::Tui,
            level_paths: vec![],
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

// TODO: read from file
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
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
                "-l" | "--level" => config.level_paths.push(parse_arg(args.next(), arg.as_str())?),

                "-m" | "--mode" => config.app_mode = parse_arg(args.next(), arg.as_str())?,
                "-r" | "--run" => config.program_mode = parse_arg(args.next(), arg.as_str())?,

                _ => return Err(format!("Unrecognized option `{arg}`!")),
            }
        }

        match config.level_paths.first() {
            Some(_) => Ok(config),
            None => Err("Provide at least one level path!".into()),
        }
    }
}
