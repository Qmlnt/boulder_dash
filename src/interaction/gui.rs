use super::{Drawable, Input, Interaction};
use crate::{args::Config, objects::Labels};
use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
    EventPump,
};
use std::{collections::BTreeMap, fs};

pub struct Gui {
    ttf_context: Sdl2TtfContext,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    texture_cache: BTreeMap<String, Box<[u8]>>,
}

impl Gui {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let canvas = sdl_context
            .video()?
            .window("Boulder Dash", 0, 0)
            //.fullscreen_desktop()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        let mut texture_cache = BTreeMap::new();
        for path in fs::read_dir("assets/sprites/")
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
        {
            let contents = fs::read(path.path())
                .map_err(|e| e.to_string())?
                .into_boxed_slice();
            texture_cache.insert(path.file_name().into_string().expect("str path"), contents);
        }

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            ttf_context,
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
                    Keycode::Comma => Input::Comma,
                    Keycode::Period => Input::Period,
                    Keycode::Q => Input::Q,
                    Keycode::P => Input::R,

                    Keycode::W => Input::W,
                    Keycode::A => Input::A,
                    Keycode::R => Input::S,
                    Keycode::S => Input::D,
                    Keycode::Up => Input::Up,
                    Keycode::Down => Input::Down,
                    Keycode::Left => Input::Left,
                    Keycode::Right => Input::Right,
                    _ => input,
                },

                _ => input,
            }
        }

        input
    }

    fn draw(&mut self, drawable: &mut impl Drawable, config: &Config) -> Result<(), String> {
        // self.canvas.set_draw_color(Color::RGB(0, 122, 255));
        self.canvas.clear();

        for (row, y) in drawable.get_objects().iter().zip(0i32..) {
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

        let (x, y) = *drawable.get_cursor();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for i in 0..config.size % 6 {
            self.canvas.draw_rect(Rect::new(
                x as i32 * i32::from(config.size) + i as i32,
                y as i32 * i32::from(config.size) + i as i32,
                u32::from(config.size).saturating_sub(2 * i as u32),
                u32::from(config.size).saturating_sub(2 * i as u32),
            ))?;
        }

        let mut bottom = drawable.get_objects().len() as i32;
        for line in drawable.get_status(config).lines() {
            let font_surface = self
                .ttf_context
                .load_font("assets/OpenSans.ttf", config.size)?
                .render(line)
                .blended(Color::RGB(200, 255, 0))
                .map_err(|e| e.to_string())?;
            let font_texture = self
                .texture_creator
                .create_texture_from_surface(&font_surface)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = font_texture.query();
            let pos = Rect::new(0, bottom * config.size as i32, width, height);
            self.canvas.copy(&font_texture, None, Some(pos))?;
            bottom += 1;
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.present();

        Ok(())
    }
}
