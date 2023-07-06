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
            thread::sleep(std::time::Duration::from_millis(1500));
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

use crate::app::AppRequest;
use std::io::{prelude::*, Write};
use std::net::TcpStream;

pub fn start_client() -> io::Result<()> {
    for i in 0..10 {
        let mut stream = TcpStream::connect("127.0.0.1:2434")?;
        let action = if i == 9 {
            AppRequest::GetKeyboard
        } else {
            AppRequest::Ping
        };
        let mut input = serde_json::to_string(&action).unwrap();
        stream.write_all(input.as_bytes())?;
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\0', &mut buffer)?;
        println!("read from server:{}", std::str::from_utf8(&buffer).unwrap());
    	stream.shutdown(std::net::Shutdown::Both)?;
    }
    Ok(())
}
