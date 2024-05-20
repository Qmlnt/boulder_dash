use std::io;

use crate::{Level, LevelObject, LevelObj};
use console::Term;
// use crate::Point;

pub enum Draw<'a> {
    Win,
    Lose,
    Init,
    Pause,
    Status(&'a [String]),
    Damaged(Vec<((usize, usize), Option<&'a LevelObject>)>),
}

pub fn tui(draw: Draw) -> io::Result<()> {
    let out = Term::stdout();

    match draw {
        Draw::Init => {
            out.hide_cursor()?;
            out.clear_screen()?;
        }
        Draw::Status(lines) => {
            out.move_cursor_down(999)?; // bottom
            for line in lines.iter().rev() {
                out.clear_line()?;
                out.write_line(line)?;
                out.move_cursor_up(1)?;
            }
        }
        Draw::Damaged(damaged) => {
            for ((x, y), obj) in damaged {
                // out.move_cursor_to(0, 0)?;
                out.move_cursor_to(x, y)?;
                out.write_line(obj.map_or("　", |o| o.char()))?;
            }
        }
        Draw::Win | Draw::Lose => out.show_cursor().unwrap(),
        Draw::Pause => {}
    };

    Ok(())
}

pub fn gui(level: &Level) {
    todo!();
}
