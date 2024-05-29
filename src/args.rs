use std::time::Duration;

const HELP_MSG: &str = "\
FLAGS:
    -g, --gui
        Launch GUI mode.
    -t, --tui
        Launch TUI mode. (default)
    -c, --cli
        TUI mode with damaged tracking.
    -p, --pause
        Launch paused.
OPTIONS:
    -d, --delay <integer>
        Delay between frames. (default: 1000ms)
    -h, --help
        Show this message.\
";

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    Gui,
    Tui,
    Cli,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub pause: bool,
    pub app_mode: AppMode,
    pub delay: Duration,
    pub level_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pause: false,
            level_paths: vec![],
            app_mode: AppMode::Tui,
            delay: Duration::from_millis(1000),
        }
    }
}

// TODO: read from file
impl Config {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut cfg = Self::default();

        let parse_num = |var_str: Option<String>, var_name| {
            var_str.map_or_else(
                || Err(format!("Missing `{var_name}` value!")),
                |num_str| {
                    num_str
                        .parse()
                        .map_or_else(|_| Err(format!("Invalid `{var_name}` value!")), Ok)
                },
            )
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    println!("{HELP_MSG}");
                    std::process::exit(0);
                }
                "-p" | "--pause" => cfg.pause = true,
                "-g" | "--gui" => cfg.app_mode = AppMode::Gui,
                "-t" | "--tui" => cfg.app_mode = AppMode::Tui,
                "-c" | "--cli" => cfg.app_mode = AppMode::Cli,
                "-d" | "--delay" => {
                    cfg.delay = Duration::from_millis(parse_num(args.next(), "delay")?);
                }

                _ => cfg.level_paths.push(arg),
            }
        }

        match cfg.level_paths.first() {
            Some(_) => Ok(cfg),
            None => Err("Provide at least one level path!".into()),
        }
    }
}
