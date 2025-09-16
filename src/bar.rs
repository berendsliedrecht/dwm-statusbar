use core::fmt::Display;

use crate::{battery::Battery, brightness::Brightness, sound::Sound, spotify::Song, time::Time};
pub struct Bar {
    song: Option<Song>,
    sound: Option<Sound>,
    brightness: Option<Brightness>,
    battery: Option<Battery>,
    time: Time,
}

impl Bar {
    pub fn new() -> Bar {
        let song = Song::new();
        let time = Time::new();
        let sound = Sound::new();
        let brightness = Brightness::new();
        let battery = Battery::new();

        Self {
            song: song.ok(),
            sound: sound.ok(),
            brightness: brightness.ok(),
            battery: battery.ok(),
            time,
        }
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let song_string = match &self.song {
            Some(s) => format!("[S: {}]", s),
            None => "".to_owned(),
        };

        let sound_string = match &self.sound {
            Some(s) => format!(" [V: {}]", s),
            None => "".to_owned(),
        };

        let battery_string = match &self.battery {
            Some(b) => format!(" [{}]", b),
            None => "".to_owned(),
        };

        let brightness_string = match &self.brightness {
            Some(b) => format!(" [B: {}]", b),
            None => "".to_owned(),
        };

        let time_string = format!(" [T: {}]", self.time);

        write!(
            f,
            "{}{}{}{}{}\0",
            song_string, brightness_string, battery_string, sound_string, time_string
        )
    }
}
