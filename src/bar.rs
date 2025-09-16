use core::fmt::Display;

use crate::{sound::Sound, spotify::Song, time::Time};

pub struct Bar {
    song: Option<Song>,
    sound: Option<Sound>,
    time: Time,
}

impl Bar {
    pub fn new() -> Bar {
        let song = Song::new();
        let time = Time::new();
        let sound = Sound::new();

        Self {
            song: song.ok(),
            sound: sound.ok(),
            time,
        }
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let song_string = match &self.song {
            Some(s) => s.to_string(),
            None => "".to_owned(),
        };

        let sound_string = match &self.sound {
            Some(s) => s.to_string(),
            None => "".to_owned(),
        };

        write!(
            f,
            "[S: {}] [V: {}] [T: {}]\0",
            song_string, sound_string, self.time
        )
    }
}
