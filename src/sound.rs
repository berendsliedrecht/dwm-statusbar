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

        let front_left = stdout.split('\n').collect::<Vec<&str>>()[5];
        let words = front_left.split_whitespace().collect::<Vec<&str>>();
        let volume = words[4].replace(['[', ']'], "");
        let audible = words[5].replace(['[', ']'], "");

        if audible == "off" {
            return Ok(Self {
                volume: "MUTED".to_owned(),
            });
        }

        Ok(Self { volume })
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
