use crate::math::rect2::Rect2;
use crate::math::vec2::Vec2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, TextureQuery, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::{EventPump, VideoSubsystem};
use crate::AudioManager;

pub struct WindowResolution(pub u32, pub u32);

impl WindowResolution {
    pub fn to_floats(&self) -> (f64, f64) {
        (self.0 as f64, self.1 as f64)
    }
}

pub struct Window<'a> {
    resolution: WindowResolution,
    sdl_canvas: WindowCanvas,
    sdl_event_pump: &'a mut EventPump,
    font: &'a Font<'a, 'a>,
    player_surface: &'a Surface<'a>,
    audio_manager: &'a AudioManager<'a>
}

impl<'a> Window<'a> {
    pub fn init(
        sdl_event_pump: &'a mut EventPump,
        audio_manager: &'a AudioManager,
        sdl_video_subsystem: &'a VideoSubsystem,
        font: &'a Font,
        player_surface: &'a Surface,
        resolution: WindowResolution,
    ) -> Window<'a> {
        let sdl_window = sdl_video_subsystem
            .window("Aerial Rust", resolution.0, resolution.1)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let sdl_canvas = sdl_window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        Window {
            resolution,
            sdl_canvas,
            sdl_event_pump,
            font,
            player_surface,
            audio_manager
        }
    }

    pub fn poll_events<T>(&mut self, f: T)
    where
        T: Fn(Event),
    {
        for event in self.sdl_event_pump.poll_iter() {
            f(event);
        }
    }

    pub fn clear_canvas(&mut self) {
        self.sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.sdl_canvas.clear();
    }

    pub fn draw_rect(&mut self, r: Rect2, color_rgb: Option<(u8, u8, u8)>) {
        let color = match color_rgb {
            None => Color::RGB(255, 255, 255),
            Some((r, g, b)) => Color::RGB(r, g, b),
        };
        self.sdl_canvas.set_draw_color(color);
        let sdl_rect = sdl2::rect::Rect::new(r.x as i32, r.y as i32, r.w as u32, r.h as u32);
        self.sdl_canvas.fill_rect(sdl_rect).unwrap();
    }

    pub fn draw_line(
        &mut self,
        a_position: &Vec2,
        b_position: &Vec2,
        color_rgb: Option<(u8, u8, u8)>,
    ) {
        let color = match color_rgb {
            None => Color::RGB(255, 255, 255),
            Some((r, g, b)) => Color::RGB(r, g, b),
        };
        self.sdl_canvas.set_draw_color(color);
        let sdl_p1 = Point::new(a_position.x as i32, a_position.y as i32);
        let sdl_p2 = Point::new(b_position.x as i32, b_position.y as i32);
        self.sdl_canvas.draw_line(sdl_p1, sdl_p2).unwrap();
    }

    pub fn fade_overlay(&mut self, opacity: f64) {
        let window_width = self.resolution.0;
        let window_height = self.resolution.1;
        let opacity_8 = (opacity * 255.0) as u8;
        let old_blend_move = self.sdl_canvas.blend_mode();
        self.sdl_canvas.set_blend_mode(BlendMode::Blend);
        self.sdl_canvas
            .set_draw_color(Color::RGBA(0, 0, 0, opacity_8));
        let sdl_rect = Rect::new(0, 0, window_width, window_height);
        self.sdl_canvas.fill_rect(sdl_rect).unwrap();
        self.sdl_canvas.set_blend_mode(old_blend_move);
    }

    pub fn get_resolution(&self) -> &WindowResolution {
        &self.resolution
    }

    pub fn present(&mut self) {
        self.sdl_canvas.present();
    }

    pub fn get_rect(&self) -> Rect2 {
        Rect2 {
            x: 0.0,
            y: 0.0,
            w: self.resolution.0 as f64,
            h: self.resolution.1 as f64,
        }
    }

    pub fn draw_text(&mut self, text: String) {
        let surface = self
            .font
            .render(text.as_str())
            .solid(Color::RGBA(255, 255, 255, 128))
            .map_err(|err| err.to_string())
            .unwrap();

        self.draw_surface(&surface, &Vec2 { x: 30.0, y: 0.0 }, None);
    }

    pub fn draw_player_surface(&mut self, pos: &Vec2, size: Option<&Vec2>) {
        self.draw_surface(self.player_surface, pos, size);
    }

    pub fn draw_surface(&mut self, surface: &Surface, pos: &Vec2, size: Option<&Vec2>) {
        let texture_creator = self.sdl_canvas.texture_creator();

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|err| err.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let (w, h) = match size {
            None => (width, height),
            Some(vec) => (vec.x as u32, vec.y as u32),
        };
        let target = Rect::new(pos.x as i32, pos.y as i32, w, h);

        self.sdl_canvas.copy(&texture, None, Some(target)).unwrap();
    }

    pub fn get_audio_manager(&self) -> &AudioManager {
        self.audio_manager
    }
}
