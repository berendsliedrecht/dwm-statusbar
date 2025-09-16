use bar::Bar;
use std::time::Duration;

mod bar;
mod sound;
mod spotify;
mod time;

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(screen: usize) -> usize;
    fn XDefaultRootWindow(display: usize) -> usize;
    fn XStoreName(display: usize, window: usize, name: *const u8) -> i32;
    fn XFlush(display: usize) -> i32;
}

fn main() -> std::io::Result<()> {
    let display = unsafe { XOpenDisplay(0) };
    let window = unsafe { XDefaultRootWindow(display) };

    loop {
        let bar = Bar::new().to_string();
        unsafe { XStoreName(display, window, bar.as_ptr()) };
        unsafe { XFlush(display) };
        std::thread::sleep(Duration::from_secs(1));
    }
}
