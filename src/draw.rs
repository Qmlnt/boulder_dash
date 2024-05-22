use console::Term;
use std::io;

use super::level;
use level::Obj;

pub fn tui(level: &level::Update) -> io::Result<()> {
    let out = Term::stdout();

    out.clear_screen()?;
    out.hide_cursor()?;

    for row in level.matrix {
        let mut line = String::new();
        for obj in row {
            line.push(obj.char());
        }
        out.write_line(&line)?;
    }

    let status_msg = match level.state {
        Some(level::State::Win) => "You have won!".to_string(),
        Some(level::State::Lose) => "You have lost!".to_string(),
        None => format!("\nScore: {}/{}", level.score, level.max_score),
    };

    out.write_line(&status_msg)?;

    // out.move_cursor_down(999)?; // bottom
    // out.write_line(&lines)?;
    // out.move_cursor_up(1)?;
    // out.move_cursor_to(0, 0)?;
    // out.move_cursor_to(x, y)?;
    // out.write_line(obj.map_or(" ", |o| o.char()))?;

    Ok(())
}

// pub fn gui(level: &Update) {
//     todo!();
// }
