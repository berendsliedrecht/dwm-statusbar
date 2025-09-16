use core::fmt;

pub struct Time {
    time: String,
}

impl Time {
    pub fn new() -> Self {
        Self {
            time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Time::new().time)
    }
}
