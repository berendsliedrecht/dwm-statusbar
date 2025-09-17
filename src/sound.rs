use std::{fmt::Display, io, process::Command};

pub struct Sound {
    volume: String,
}

impl Sound {
    pub fn new() -> io::Result<Self> {
        let status = Command::new("amixer")
            .arg("get")
            .arg("Master")
            .output()?
            .stdout;

        let stdout = std::str::from_utf8(&status).unwrap();

        let tokens = stdout.split('\n').collect::<Vec<&str>>();

        let is_mono = tokens.iter().any(|t| t.contains("Mono: Playback"));

        let (volume, audible) = if is_mono {
            let mono = tokens[4];
            let tokens: Vec<_> = mono.split_whitespace().collect();
            let volume = tokens[3];
            let audible = tokens[5];
            let volume = volume.replace(['[', ']'], "");

            (volume, audible == "[on]")
        } else {
            let front_left = tokens[5];
            let tokens: Vec<_> = front_left.split_whitespace().collect();
            let volume = tokens[4];
            let audible = tokens[5];
            let volume = volume.replace(['[', ']'], "");

            (volume, audible == "[on]")
        };

        if audible {
            Ok(Self { volume })
        } else {
            Ok(Self {
                volume: "MUTED".to_owned(),
            })
        }
    }
}

impl Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Sound::new()
                .unwrap_or(Self {
                    volume: "e".to_owned()
                })
                .volume
        )
    }
}
