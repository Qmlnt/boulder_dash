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
    EventPump, IntegerOrSdlError,
};
use std::{collections::BTreeMap, error::Error, fs};

pub struct Gui {
    scale: u32,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
    texture_cache: BTreeMap<String, Box<[u8]>>,
}

impl Gui {
    pub fn new(scale: u32) -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let ttf_context = sdl2::ttf::init()?;

        let canvas = sdl_context
            .video()?
            .window("Boulder Dash", 0, 0)
            .position_centered()
            .build()?
            .into_canvas()
            .software()
            .build()?;
        let event_pump = sdl_context.event_pump()?;
        let texture_creator = canvas.texture_creator();

        let mut texture_cache = BTreeMap::new();
        for path in fs::read_dir("assets/sprites/")?.filter_map(Result::ok) {
            let contents = fs::read(path.path())?.into_boxed_slice();
            texture_cache.insert(path.file_name().into_string().expect("str path"), contents);
        }

        Ok(Self {
            scale,
            canvas,
            event_pump,
            ttf_context,
            texture_creator,
            texture_cache,
        })
    }

    fn resize_window(&mut self, (width, height): (u32, u32)) -> Result<(), IntegerOrSdlError> {
        self.canvas.window_mut().set_minimum_size(width, height)?;
        self.canvas.window_mut().set_maximum_size(width, height)
        // self.canvas.window_mut().set_size(width, height)?;
        // self.canvas.set_logical_size(width, height)
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
        self.canvas.set_draw_color(Color::BLACK);
        // Redraw objects using the damaged buffer
        let mut objects_to_redraw = drawable.get_damaged();

        // WINDOW

        let drawable_size = (
            u32::try_from(drawable.get_width())? * self.scale,
            // scale + 1 is padding for the status
            u32::try_from(drawable.get_height())? * (self.scale + 1),
        );
        if drawable_size != self.canvas.window().size() {
            // TODO: why it takes 2 calls to resize normally
            self.resize_window(drawable_size)?;
            self.resize_window(drawable_size)?;
            self.canvas.clear(); // clear the artifacts after resize

            // Redraw all objects
            objects_to_redraw = drawable
                .get_objects()
                .iter()
                .enumerate()
                .flat_map(|(y, row)| (0..row.len()).map(move |x| (x, y)))
                .collect();
        }

        // OBJECTS

        for (x, y) in objects_to_redraw {
            let rect = Rect::new(
                i32::try_from(x)? * i32::try_from(self.scale)?,
                i32::try_from(y)? * i32::try_from(self.scale)?,
                self.scale,
                self.scale,
            );
            self.canvas.fill_rect(rect)?; // clear the old artifacts

            if let Some(obj) = drawable.get_object((x, y)) {
                let bytes = &self.texture_cache[&obj.name()];
                let texture = self.texture_creator.load_texture_bytes(bytes)?;
                self.canvas.copy(&texture, None, rect)?;
            }
        }

        // STATUS

        let font = self
            .ttf_context
            .load_font("assets/font.ttf", u16::try_from(self.scale)?)?;
        let mut level_bottom = u32::try_from(drawable.get_objects().len())? * self.scale;

        // Clear the bottom of the screen
        self.canvas.fill_rect(Rect::new(
            0,
            i32::try_from(level_bottom)?,
            drawable_size.0,
            drawable_size.1.saturating_sub(level_bottom),
        ))?;

        // Draw the status line by line
        for line in drawable.get_status().lines() {
            let font_surface = font.render(line).blended(Color::RGB(200, 255, 0))?;
            let font_texture = self
                .texture_creator
                .create_texture_from_surface(&font_surface)?;

            let TextureQuery { width, height, .. } = font_texture.query();
            let rect = Rect::new(0, i32::try_from(level_bottom)?, width, height);
            self.canvas.copy(&font_texture, None, rect)?;

            level_bottom += self.scale;
        }

        // CURSOR

        if let Some(&(x, y)) = drawable.get_cursor() {
            self.canvas.set_draw_color(Color::RGB(0, 255, 0));

            for i in 0..self.scale / 6 {
                self.canvas.draw_rect(Rect::new(
                    i32::try_from(x)? * i32::try_from(self.scale)? + i32::try_from(i)?,
                    i32::try_from(y)? * i32::try_from(self.scale)? + i32::try_from(i)?,
                    self.scale.saturating_sub(2 * i),
                    self.scale.saturating_sub(2 * i),
                ))?;
            }
        }

        // Displaying the backbuffer

        // TODO: why 3 calls?
        self.canvas.present();
        self.canvas.present();
        self.canvas.present();

        Ok(())
    }
}
