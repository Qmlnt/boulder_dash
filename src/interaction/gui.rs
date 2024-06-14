use super::{Drawable, Input, Interaction};
use crate::objects::Labels;
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
use std::{collections::BTreeMap, error::Error, fs};

pub struct Gui {
    size: u16,
    ttf_context: Sdl2TtfContext,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    texture_cache: BTreeMap<String, Box<[u8]>>,
}

impl Gui {
    pub fn new(size: u16) -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let ttf_context = sdl2::ttf::init()?;

        let canvas = sdl_context
            .video()?
            .window("Boulder Dash", 1000, 1000)
            // .fullscreen_desktop()
            .position_centered()
            .build()?
            .into_canvas()
            .software()
            .build()?;

        let event_pump = sdl_context.event_pump()?;

        let mut texture_cache = BTreeMap::new();
        for path in fs::read_dir("assets/sprites/")?.filter_map(Result::ok) {
            let contents = fs::read(path.path())?.into_boxed_slice();
            texture_cache.insert(path.file_name().into_string().expect("str path"), contents);
        }

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            size,
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
                Event::Quit { .. } => return Input::Quit,

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

    fn draw(&mut self, drawable: &mut impl Drawable) -> Result<(), Box<dyn Error>> {
        self.canvas.clear();

        self.canvas.window_mut().set_size(
            u32::try_from(drawable.get_width())? * u32::from(self.size),
            u32::try_from(drawable.get_height())? * u32::from(self.size) + u32::from(self.size) / 4,
        )?;

        for (y, row) in drawable.get_objects().iter().enumerate() {
            for (x, obj) in row.iter().enumerate() {
                let pos = Rect::new(
                    i32::try_from(x)? * i32::from(self.size),
                    i32::try_from(y)? * i32::from(self.size),
                    u32::from(self.size),
                    u32::from(self.size),
                );
                let bytes = &self.texture_cache[&obj.name()];
                let texture = self.texture_creator.load_texture_bytes(bytes)?;
                self.canvas.copy(&texture, None, pos)?;
            }
        }

        if let Some(&(x, y)) = drawable.get_cursor() {
            self.canvas.set_draw_color(Color::RGB(0, 255, 0));

            for i in 0..self.size / 6 {
                self.canvas.draw_rect(Rect::new(
                    i32::try_from(x)? * i32::from(self.size) + i32::from(i),
                    i32::try_from(y)? * i32::from(self.size) + i32::from(i),
                    u32::from(self.size.saturating_sub(2 * i)),
                    u32::from(self.size.saturating_sub(2 * i)),
                ))?;
            }
        }

        let font = self.ttf_context.load_font("assets/font.ttf", self.size)?;
        let mut level_bottom = i32::try_from(drawable.get_objects().len())?;

        for line in drawable.get_status().lines() {
            let font_surface = font.render(line).blended(Color::RGB(200, 255, 0))?;
            let font_texture = self
                .texture_creator
                .create_texture_from_surface(&font_surface)?;

            let TextureQuery { width, height, .. } = font_texture.query();
            let pos = Rect::new(5, level_bottom * i32::from(self.size), width, height);
            self.canvas.copy(&font_texture, None, Some(pos))?;

            level_bottom += 1;
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.present();

        Ok(())
    }
}
