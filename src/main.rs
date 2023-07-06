#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![windows_subsystem = "windows"]

use glam::UVec2;

use speedy2d::{
    window::{WindowCreationOptions, WindowPosition, WindowSize},
    Window,
};

mod app;
use app::App;

mod config;
use config::Config;

mod font;
mod game;
mod screenshot;
mod spritesheet;

use std::io::{self, BufRead, BufReader};
use std::sync::mpsc;
use std::thread;

fn main() {
    thread::Builder::new()
        .name("app_client thread".to_string())
        .spawn(move || {
            thread::sleep(std::time::Duration::from_millis(500));
            start_client().unwrap();
        });
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        tx.send("started loop".to_string()).unwrap();
        loop {
            let reader = BufReader::new(io::stdin().lock());
            for line in reader.lines().filter_map(|line| line.ok()) {
                let res = tx.send(line);
                res.unwrap();
            }
        }
    });

    let config = Config::new("config.txt");
    let window_size = UVec2::new(config.window_width, config.window_height);
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        &config.title,
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(true)
            .with_transparent(false),
    )
    .expect("Wasn't able to create a window!");
    window.run_loop(App::new(window_size, config, rx));
}

use crate::app::{AppRequest, Keyboard};
use speedy2d::window::VirtualKeyCode;
use std::io::{prelude::*, Write};
use std::net::TcpStream;

pub fn start_client() -> io::Result<()> {
    loop {
        let mut stream = TcpStream::connect("127.0.0.1:2434")?;
        let action = AppRequest::GetKeyboard;
        stream.write_all(serde_json::to_string(&action).unwrap().as_bytes())?;
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\0', &mut buffer);
        let result = std::str::from_utf8(&buffer).unwrap();
        let kbd: Keyboard = serde_json::from_str(result).unwrap();

        stream.shutdown(std::net::Shutdown::Both)?;
        let mut stream = TcpStream::connect("127.0.0.1:2434")?;
        
        let game_command = if kbd.pressed.contains(&VirtualKeyCode::U) {
            "r"
        } else if kbd.pressed.contains(&VirtualKeyCode::O) {
            "l"
        } else if kbd.pressed.contains(&VirtualKeyCode::Period) {
            "u"
        } else if kbd.pressed.contains(&VirtualKeyCode::E) {
            "d"
        } else { "w" };
		let action = AppRequest::Command(game_command.to_string());
        stream.write_all(serde_json::to_string(&action).unwrap().as_bytes())?;
        stream.shutdown(std::net::Shutdown::Both)?;
    }
    Ok(())
}
