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

mod server;
use server::spawn_server_thread;

fn main() {

    thread::spawn(move || {
    	spawn_server_thread();
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
