const HELP_MSG: &str = "\
FLAGS:
    -g, --gui
        Launch GUI mode.
    -t, --tui
        Launch TUI mode. (default)
OPTIONS:
    -d, --delay <integer>
        Delay between frames. (default: 1000ms)
    -h, --help
        Show this message.\
";

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Tui,
    Gui,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub delay: u64,
    pub app_mode: AppMode,
    pub level_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 1000,
            app_mode: AppMode::Tui,
            level_paths: Vec::new(),
        }
    }
}

impl Config {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut cfg = Self::default();

        let parse_num = |num: Option<String>, name| match num {
            Some(val_str) => match val_str.parse::<u64>() {
                Ok(val) => Ok(val),
                Err(_) => Err(format!("Invalid `{name}` value!")),
            },
            None => Err(format!("Missing `{name}` value!")),
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    println!("{HELP_MSG}");
                    std::process::exit(0);
                }
                "-g" | "--gui" => cfg.app_mode = AppMode::Gui,
                "-t" | "--tui" => cfg.app_mode = AppMode::Tui,
                "-d" | "--delay" => cfg.delay = parse_num(args.next(), "delay")?,
                _ => cfg.level_paths.push(arg),
            }
        }

        match cfg.level_paths.first() {
            Some(_) => Ok(cfg),
            None => Err("Provide at least one level path!".into()),
        }
    }
}
