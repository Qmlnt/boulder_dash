use console::Term;
use std::error::Error;
use std::fs;
use std::sync::mpsc;
use std::{thread, time::Duration};

mod args;
mod draw;
pub use args::Config;

#[derive(Default, Debug)]
struct Level {
    gems: u8,
    score: u8,
    matrix: Vec<Vec<Obj>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Obj {
    Gem,
    Void,
    Dirt,
    Rock,
    Wall,
    Player,
}

impl Level {
    fn new(contents: String) -> Result<Level, String> {
        let mut level = Level::default();

        for line in contents.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let obj = match c {
                    'g' => {
                        level.gems += 1;
                        Obj::Gem
                    }
                    ' ' => Obj::Void,
                    'd' => Obj::Dirt,
                    'r' => Obj::Rock,
                    'p' => Obj::Player,
                    '#' | '|' | '-' => Obj::Wall,
                    _ => return Err(format!("Can't parse char `{c}`")),
                };
                row.push(obj);
            }
            level.matrix.push(row);
        }

        Ok(level)
    }

    pub fn tick(&mut self, mr: i32, mut mc: i32) -> bool {
        let mat = &mut self.matrix;
        let mut checked_player = false;

        for r in (0..mat.len()).rev() {
            for c in 0..mat.len() {
                if mat[r][c] == Obj::Player && !checked_player {
                    checked_player = true;

                    let [nr, nc] = [(r as i32 + mr) as usize, (c as i32 + mc) as usize];
                    let [nnr, nnc] = [(nr as i32 + mr) as usize, (nc as i32 + mc) as usize];

                    // can move if not moving into a wall or an unmovable (horizontally!) rock.
                    if [mr, mc] != [0, 0]
                        && mat[nr][nc] != Obj::Wall
                        && (mat[nr][nc] != Obj::Rock || (mr == 0 && mat[nnr][nnc] == Obj::Void))
                    {
                        mat[r][c] = Obj::Void;
                        if mat[nr][nc] == Obj::Rock {
                            mat[nnr][nnc] = Obj::Rock;
                        } else if mat[nr][nc] == Obj::Gem {
                            self.score += 1;
                        }
                        mat[nr][nc] = Obj::Player;
                    } else if mat[r - 1][c] == Obj::Rock {
                        mat[r][c] = Obj::Rock;
                        mat[r - 1][c] = Obj::Void;
                        return false;
                    }
                } else if mat[r][c] == Obj::Rock {
                    mat[r][c] = Obj::Void;

                    let mut c = c;
                    if !checked_player && mc == -1 && mat[r][c + 1] == Obj::Player {
                        if mat[r][c - 1] == Obj::Void {
                            c -= 1;
                        } else {
                            mc = 0;
                        }
                    }

                    if mat[r + 1][c] == Obj::Void {
                        mat[r + 1][c] = Obj::Rock;
                    } else if mat[r][c + 1] == Obj::Void && mat[r + 1][c + 1] == Obj::Void {
                        mat[r][c + 1] = Obj::Rock;
                    } else if mat[r][c - 1] == Obj::Void && mat[r + 1][c - 1] == Obj::Void {
                        mat[r][c - 1] = Obj::Rock;
                    } else {
                        mat[r][c] = Obj::Rock;
                    }
                }
            }
        }

        true
    }
}

// Main game loop
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut levels = Vec::new();

    for level_path in config.level_paths.iter() {
        let contents = fs::read_to_string(level_path)?;
        levels.push(Level::new(contents)?);
    }

    if config.app_mode == args::AppMode::Gui {
        return Err("Not yet implemented!".into());
    }

    // to not block the main thread while waiting for a key
    let mut stdout = Term::stdout();
    let (tx, rx) = mpsc::channel();

    let stdout2 = stdout.clone();
    thread::spawn(move || loop {
        let key = stdout2.read_char().unwrap();
        tx.send(key).unwrap();
    });

    // main loop
    for mut level in levels {
        while {
            // while {...} {} = do while
            draw::tui(&level, &mut stdout);
            if level.score == level.gems {
                false
            } else {
                thread::sleep(Duration::from_millis(config.delay));
                let mut chr = ' '; // to get the last char
                while let Ok(c) = rx.try_recv() {
                    chr = c;
                }
                let dir = match chr { // TODO add keys to cmd args
                    'w' => [-1, 0],
                    'r' => [1, 0],
                    'a' => [0, -1],
                    's' => [0, 1],
                    _ => [0, 0],
                };
                level.tick(dir[0], dir[1])
            }
        } {}
        draw::tui(&level, &mut stdout);

        if level.score != level.gems {
            stdout.clear_last_lines(1)?;
            println!("You lost!");
            return Ok(());
        }
    }

    stdout.clear_last_lines(1)?;
    println!("You won!");

    Ok(())
}
