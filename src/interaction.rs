use console::{Key, Term};
use enum_dispatch::enum_dispatch;
use std::{error::Error, sync::mpsc, thread};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

use crate::{
    level::{Obj, State, Update},
    Config,
};

pub enum Input {
    Esc,
    Quit,
    Reload,
    Unknown,

    Up,
    Down,
    Left,
    Right,
    Space,
    DelayDown,
    DelayUp,
}

#[enum_dispatch]
pub enum Mode {
    Tui,
    Gui,
}

#[enum_dispatch(Mode)]
pub trait Interaction {
    fn get_input(&mut self) -> Input;
    fn draw(&mut self, level: &Update, config: &Config) -> Result<(), Box<dyn Error>>;
}

pub struct Gui {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
}

impl Gui {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        let window = sdl_context
            .video()?
            .window("Boulder Dash", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            canvas,
            event_pump,
            texture_creator,
        })
    }
}

impl Interaction for Gui {
    fn get_input(&mut self) -> Input {
        let mut input = Input::Unknown;

        while let Some(event) = self.event_pump.poll_event() {
            input = match event {
                Event::Quit { .. } => return Input::Quit,

                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => Input::Esc,
                    Keycode::Space => Input::Space,
                    Keycode::P => Input::Reload,
                    Keycode::Comma => Input::DelayDown,
                    Keycode::Period => Input::DelayUp,
                    Keycode::Q => Input::Quit,
                    Keycode::W | Keycode::Up => Input::Up,
                    Keycode::R | Keycode::Down => Input::Down,
                    Keycode::A | Keycode::Left => Input::Left,
                    Keycode::S | Keycode::Right => Input::Right,
                    _ => Input::Unknown,
                },

                _ => input,
            }
        }

        input
    }

    fn draw(&mut self, level: &Update, _config: &Config) -> Result<(), Box<dyn Error>> {
        //self.canvas.set_draw_color(Color::RGB(0, 122, 255));
        self.canvas.clear();

        for (row, y) in level.matrix.iter().zip(0i32..) {
            for (obj, x) in row.iter().zip(0i32..) {
                let texture = self
                    .texture_creator
                    .create_texture_from_surface(obj.sprite()?)?;
                let pos = Rect::new(x * 30, y * 30, 30, 30);
                self.canvas.copy(&texture, None, pos)?;
            }
        }

        self.canvas.present();

        Ok(())
    }
}

pub struct Tui {
    term: Term,
    input_rx: mpsc::Receiver<Key>,
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

impl Tui {
    pub fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel();

        let term = Term::stdout();
        let stdin = term.clone();
        thread::spawn(move || loop {
            let key = stdin.read_key().expect("Should always get a key");
            input_tx.send(key).expect("Receiver should be present");
        });

        Self { term, input_rx }
    }
}


impl Interaction for Tui {
    fn get_input(&mut self) -> Input {
        let input = self.input_rx.try_recv();
        input.map_or(Input::Unknown, |key| match key {
            Key::Escape => Input::Esc,
            Key::Char(' ') => Input::Space,
            Key::Char('p') => Input::Reload,
            Key::Char(',') => Input::DelayDown,
            Key::Char('.') => Input::DelayUp,
            Key::Char('q') | Key::CtrlC => Input::Quit,
            Key::Char('w') | Key::ArrowUp => Input::Up,
            Key::Char('r') | Key::ArrowDown => Input::Down,
            Key::Char('a') | Key::ArrowLeft => Input::Left,
            Key::Char('s') | Key::ArrowRight => Input::Right,
            _ => Input::Unknown,
        })
    }

    fn draw(&mut self, level: &Update, config: &Config) -> Result<(), Box<dyn Error>> {
        self.term.clear_screen()?;
        self.term.hide_cursor()?;

        for row in level.matrix {
            let mut line = String::new();
            for obj in row {
                line.push(obj.char());
            }
            self.term.write_line(&line)?;
        }

        let status_msg = match level.state {
            Some(State::Win) => "You have won!".to_string(),
            Some(State::Lose) => "You have lost!".to_string(),
            None => format!(
                "\nScore: {}/{}\nDelay: {}ms\nPaused: {}",
                level.score,
                level.max_score,
                config.delay.as_millis(),
                if config.pause { "yes" } else { "no" }
            ),
        };
        self.term.write_line(&status_msg)?;

        Ok(())
    }
}
