use std::path::Path;
use sdl2::mixer::{AUDIO_S16LSB, Channel, Chunk, DEFAULT_CHANNELS, InitFlag, Music, Sdl2MixerContext};
use sdl2::Sdl;

pub struct AudioManager<'a> {
    sdl_context: &'a Sdl,
    music: Music<'a>,
    blip: Chunk,
}

impl<'a> AudioManager<'a> {
    pub fn init(sdl_context: &'a Sdl) -> Self {

        sdl_context.audio().unwrap();
        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
        sdl2::mixer::init(InitFlag::MP3).unwrap();
        sdl2::mixer::allocate_channels(32);

        let music_path = Path::new("assets/bg-music.mp3");
        if !music_path.exists() {
            panic!("music path does not exist: {}", music_path.to_string_lossy())
        }

        let blip_path = Path::new("assets/blip2.mp3");
        if !blip_path.exists() {
            panic!("blip path does not exist: {}", blip_path.to_string_lossy())
        }

        let music = Music::from_file(music_path).unwrap();
        let blip = Chunk::from_file(blip_path).unwrap();

        AudioManager{
            sdl_context,
            music,
            blip
        }

    }

    pub fn play_music(&self) {
        self.music.play(-1).unwrap();
    }

    pub fn play_blip(&self) {
        Channel::all().set_volume(20);
        Channel::all().play(&self.blip, 1).unwrap();
    }
}
