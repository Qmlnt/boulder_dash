use std::io;

use crate::Level;
use crate::LevelUpdate;
use console::Term;
// use crate::Point;

pub trait Drawable {
    fn char(&self) -> &str;
    fn sprite(&self) {
        todo!()
    }
}

pub enum Draw {
    Win,
    Lose,
    Init,
    Pause,
    Status(Vec<String>),
    Damaged(Vec<(usize, usize, Box<dyn Drawable>)>),
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
            for (x, y, obj) in damaged {
                out.move_cursor_to(0, 0)?;
                out.move_cursor_to(x, y)?;
                out.write_line(obj.char())?;
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
