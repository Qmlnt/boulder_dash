use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    gui: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let gui = env::var("GUI").is_ok();

        Ok(Config { file_path, gui })
    }
}

struct Level {
    matrix: Vec<Vec<Cell>>,
}

impl Level {
    fn load(string: String) -> Result<Level, String> {
        let mut matrix = vec![];
        for line in string.split('\n') {
            let mut row = vec![];
            for c in line.chars() {
                row.push(match c {
                    'g' => Cell::Gem,
                    '-' => Cell::Wall,
                    '|' => Cell::Wall,
                    'r' => Cell::Rock,
                    ' ' => Cell::Void,
                    'b' => Cell::Bush,
                    'p' => Cell::Player,
                    _ => return Err(format!("Can't parse char `{c}`")),
                });
            }
            matrix.push(row);
        }
        Ok(Level { matrix })
    }

    fn compose(&self) -> String {
        let mut frame = String::new();

        for row in self.matrix.iter() {
            for c in row {
                frame.push(c.char());
            }
            frame.push('\n');
        }

        frame
    }
}

enum Cell {
    Gem,
    Wall,
    Rock,
    Void,
    Bush,
    Player,
}

impl Cell {
    fn char(&self) -> char {
        match self {
            Cell::Gem => '💎',
            Cell::Wall => '🧱',
            Cell::Rock => '🪨',
            Cell::Void => ' ',
            Cell::Bush => '🌳',
            Cell::Player => '󰋦',
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.gui {
        return Err("Not yet implemented!".into());
    }

    let level = Level::load(contents)?;
    println!("{}", level.compose());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn load_level() -> Level {
        Level::load("||rbpg \n gpbr-|".to_string()).expect("level loaded")
    }

    #[test]
    fn loading_level_from_string() {
        load_level();
    }

    #[test]
    fn composing_a_level() {
        let level = load_level();
        assert_eq!(level.compose(), "🧱🧱🪨🌳󰋦💎 \n 💎󰋦🌳🪨🧱🧱\n");
    }
}
