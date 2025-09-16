use core::fmt::Display;
use std::fs::read_to_string;

pub struct Brightness {
    brightness: String,
}

impl Brightness {
    pub fn new() -> Result<Self, ()> {
        let max_brightness = read_to_string("/sys/class/backlight/intel_backlight/max_brightness")
            .map_err(|_| ())?;
        let curr_brightness =
            read_to_string("/sys/class/backlight/intel_backlight/brightness").map_err(|_| ())?;

        let max_brightness = max_brightness.trim().parse::<u16>().map_err(|_| ())?;
        let curr_brightness = curr_brightness.trim().parse::<u16>().map_err(|_| ())?;

        let brightness = (curr_brightness as f32 / max_brightness as f32) * 100.0;

        Ok(Self {
            brightness: format!("{:.0}", brightness),
        })
    }
}

impl Display for Brightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.brightness)
    }
}
