use std::error::Error;
use std::fs;

mod args;
pub use args::Config;

pub enum ObjectType {
    Gem,
    Void,
    Dirt,
    Rock,
    Wall,
    Player,
}

struct Level {
    matrix: Vec<Vec<ObjectType>>,
}

impl Default for Level {
    fn default() -> Self {
        Level { matrix: Vec::new() }
    }
}

impl Level {
    fn load(contents: String) -> Result<Level, String> {
        let mut level = Level::default();

        for line in contents.split('\n') {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    'g' => ObjectType::Gem,
                    ' ' => ObjectType::Void,
                    'd' => ObjectType::Dirt,
                    'r' => ObjectType::Rock,
                    'p' => ObjectType::Player,
                    '|' | '-' => ObjectType::Wall,
                    _ => return Err(format!("Can't parse char `{c}`")),
                });
            }
            level.matrix.push(row);
        }

        Ok(level)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let args::AppMode::GUI = config.app_mode {
        return Err("Not yet implemented!".into());
    }

    let mut levels = Vec::new();
    for level_path in config.level_paths {
        levels.push(Level::load(fs::read_to_string(level_path)?)?);
    }

    draw_tui(&levels[0]);

    Ok(())
}

fn draw_tui(level: &Level) {
    let mut frame = String::new();
    for row in level.matrix.iter() {
        for o in row {
            frame.push(match o {
                ObjectType::Gem => '💎',
                ObjectType::Void => '⬜',
                ObjectType::Dirt => '🟫',
                ObjectType::Rock => '🪨',
                ObjectType::Wall => '🧱',
                ObjectType::Player => '🤵',
            })
        }
        frame.push('\n');
    }

    println!("{frame}");
}

#[cfg(test)]
mod tests {
    use super::*;
    fn load_level() -> Level {
        Level::load("||rdpg \n gpdr-|".to_string()).expect("level loaded")
    }

    #[test]
    fn loading_level_from_string() {
        load_level();
    }

    // #[test]
    // fn composing_a_level() {
    //     let level = load_level();
    //     assert_eq!(level.compose(), "🧱🧱🪨🟫󰋦💎🔲\n🔲💎󰋦🟫🪨🧱🧱\n");
    // }
}
