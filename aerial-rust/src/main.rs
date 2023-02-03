use std::borrow::Borrow;
use std::path::Path;
use std::thread;
use crate::audio_manager::AudioManager;
use crate::game::Game;
use crate::time_manager::TimeManager;
use crate::window::{Window, WindowResolution};
use sdl2::image::{InitFlag, LoadSurface};
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag as MixerInitFlag};

mod audio_manager;
mod enemy;
mod enemy_bond;
mod game;
mod math;
mod player;
mod score;
mod time_manager;
mod traits;
mod utils;
mod window;

fn main() {
    // init SDL
    let sdl_context = sdl2::init().map_err(|e| e.to_string()).unwrap();

    // init video
    let sdl_video_subsystem = sdl_context.video().map_err(|e| e.to_string()).unwrap();

    // init audio
    let audio_manager = AudioManager::init(&sdl_context);

    // init image
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    // init event pump
    let mut sdl_event_pump = sdl_context.event_pump().unwrap();

    // init font
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("assets/Pixeled.ttf", 30).unwrap();

    // init window
    let player_surface = sdl2::surface::Surface::from_file("assets/jet.png").unwrap();
    let mut window = Window::init(
        &mut sdl_event_pump,
        &audio_manager,
        &sdl_video_subsystem,
        &font,
        &player_surface,
        WindowResolution(1200, 800),
    );

    // init time manager
    let mut time_manager = TimeManager::new(60);

    // init game
    let mut game = Game::init(&mut time_manager, &audio_manager, &mut window);
    game.run();
}
