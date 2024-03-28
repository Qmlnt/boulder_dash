use crate::Level;
use crate::Obj;
use console::Term;

pub fn tui(level: &Level, console: &mut Term) {
    console.clear_screen().unwrap();

    for row in level.matrix.iter() {
        let mut line = String::new();
        for obj in row {
            let chr = match obj {
                Obj::Gem => '🍏', //'💎',
                Obj::Void => '　',
                Obj::Dirt => '🟫',
                Obj::Rock => '🪨',
                Obj::Wall => '🧱',
                Obj::Player => '🐷', //'🤵',
            };
            line.push(chr);
        }
        console.write_line(&line).unwrap();
    }
    println!("\nScore: {}/{}", level.score, level.gems);
}
