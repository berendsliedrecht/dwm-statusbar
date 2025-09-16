use core::fmt;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Song {
    item: SongItem,
    is_playing: bool,
}

impl Song {
    pub fn new() -> Result<Self, ()> {
        let cmd = Command::new("spotify_player")
            .arg("get")
            .arg("key")
            .arg("playback")
            .output()
            .map_err(|_| ())?;

        let stdout = std::str::from_utf8(&cmd.stdout).map_err(|_| ())?;
        let song_info: Self = serde_json::from_str(stdout).map_err(|_| ())?;
        Ok(song_info)
    }
}

#[derive(Deserialize, Debug)]
struct SongArtist {
    name: String,
}

#[derive(Deserialize, Debug)]
struct SongItem {
    name: String,
    artists: Vec<SongArtist>,
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let artist = &self.item.artists[0].name;
        let is_playing = if self.is_playing { "" } else { " - PAUSED" };
        write!(f, "{} - {}{}", artist, self.item.name, is_playing)
    }
}
