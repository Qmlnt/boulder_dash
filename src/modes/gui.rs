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
use std::{collections::BTreeMap, error::Error, fs};

pub struct Gui {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    texture_cache: BTreeMap<String, Box<[u8]>>,
}

impl Gui {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let canvas = sdl_context
            .video()?
            .window("Boulder Dash", 800, 600)
            .fullscreen_desktop()
            .position_centered()
            .build()?
            .into_canvas()
            .software()
            .build()?;

        let event_pump = sdl_context.event_pump()?;

        let mut texture_cache = BTreeMap::new();
        for path in fs::read_dir("assets/sprites/")?.filter_map(Result::ok) {
            //let file_name = Path::new(&path.file_name()).with_extension("");
            let contents = fs::read(path.path())?.into_boxed_slice();
            texture_cache.insert(path.file_name().into_string().expect("str path"), contents);
        }

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
            event_pump,
            texture_creator,
            texture_cache,
        })
    }
}

impl Interaction for Gui {
    fn get_input(&mut self) -> Input {
        let mut input = Input::Unknown;

        while let Some(event) = self.event_pump.poll_event() {
            input = match event {
                Event::Quit { .. } => return Input::Q,

                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => Input::Esc,
                    Keycode::Space => Input::Space,
                    Keycode::P => Input::R,
                    Keycode::Comma => Input::Comma,
                    Keycode::Period => Input::Dot,
                    Keycode::Q => Input::Q,
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

    fn draw(&mut self, level: &mut Level, config: &Config) -> Result<(), Box<dyn Error>> {
        // self.canvas.set_draw_color(Color::RGB(0, 122, 255));
        self.canvas.clear();

        for (row, y) in level.get_objects().iter().zip(0i32..) {
            for (obj, x) in row.iter().zip(0i32..) {
                let pos = Rect::new(
                    x * i32::from(config.size),
                    y * i32::from(config.size),
                    u32::from(config.size),
                    u32::from(config.size),
                );
                let bytes = &self.texture_cache[&obj.name()];
                let texture = self.texture_creator.load_texture_bytes(bytes)?;
                self.canvas.copy(&texture, None, pos)?;
            }
        }

        self.canvas.present();

        Ok(())
    }
}
