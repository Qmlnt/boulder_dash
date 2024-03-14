pub struct Config {
    pub level_paths: Vec<String>,
    pub app_mode: AppMode,
}

#[derive(Debug, PartialEq)]
pub enum AppMode {
    TUI,
    GUI,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_mode: AppMode::TUI,
            level_paths: Vec::new(),
        }
    }
}

impl Config {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut cfg = Self::default();

        args.next();
        for arg in args {
            match arg.as_str() {
                "-g" | "--gui" => cfg.app_mode = AppMode::GUI,
                "-t" | "--tui" => cfg.app_mode = AppMode::TUI,
                _ => cfg.level_paths.push(arg),
            }
        }

        if cfg.level_paths.is_empty() {
            Err("Provide at least one level path!".into())
        } else {
            Ok(cfg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let args = "-t --tui /dev/zero -g --gui /dev /tmp".split(' ');
        let cfg = Config::parse(args.map(|s| s.to_string())).expect("Config struct");

        assert_eq!(cfg.app_mode, AppMode::GUI);
        assert_eq!(cfg.level_paths, vec!["/dev/zero", "/dev", "/tmp"]);
    }
}
