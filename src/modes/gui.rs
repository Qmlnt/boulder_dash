use super::{Input, Interaction, Labels};
use crate::{args::Config, level::Level};
use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};
use std::{error::Error, path::Path};

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
                    _ => input,
                },

                _ => input,
            }
        }

        input
    }

    fn draw(&mut self, level: &mut Level, _config: &Config) -> Result<(), Box<dyn Error>> {
        //self.canvas.set_draw_color(Color::RGB(0, 122, 255));
        self.canvas.clear();

        for (row, y) in level.get_objects().iter().zip(0i32..) {
            for (obj, x) in row.iter().zip(0i32..) {
                let texture = self
                    .texture_creator
                    .load_texture(Path::new(&format!("assets/sprites/{}.png", obj.name())))?;

                let pos = Rect::new(x * 30, y * 30, 30, 30);
                self.canvas.copy(&texture, None, pos)?;
            }
        }

        self.canvas.present();

        Ok(())
    }
}
