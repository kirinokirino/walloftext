use glam_rect::Rect;
use speedy2d::color::Color;
use speedy2d::image::ImageDataType;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode;
use speedy2d::Graphics2D;

use glam::{UVec2, Vec2};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::font::vga8;
use crate::spritesheet::Spritesheet;

const FILE: &'static str = include_str!("./game.rs");

pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,
    spritesheets: Vec<Spritesheet>,
    counter: usize,
    display_buffer: [[Tile; 80]; 40],

    viewport_size: UVec2,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        let buffer = [[Tile::new(' '); 80]; 40];
        Self {
            config,
            images: Vec::new(),
            spritesheets: Vec::new(),
            display_buffer: buffer,

            counter: 0,
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        let image_handle = graphics
            .create_image_from_raw_pixels(
                ImageDataType::RGBA,
                ImageSmoothingMode::NearestNeighbor,
                UVec2::new(8, 16 * 256),
                &vga8(),
            )
            .unwrap();
        self.spritesheets
            .push(Spritesheet::new(image_handle, 1, 256));

        for (y, line) in FILE.lines().enumerate() {
            self.display_string(
                &line
                    .chars()
                    .enumerate()
                    .filter_map(|(i, ch)| if i < 40 { Some(ch) } else { None })
                    .collect::<String>(),
                UVec2::new(0, y as u32),
                &Color::WHITE,
                &Color::BLACK,
            );
        }
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self, current_frame: u64) {
        self.counter += 1;
    }

    fn draw_char(
        &self,
        ch: &char,
        position: Vec2,
        color: &Color,
        bg_color: &Color,
        graphics: &mut Graphics2D,
    ) {
        let vga8 = self.spritesheets.get(0).unwrap();
        let width = self.config.grid_width;
        let height = self.config.grid_height;
        let rect = Rect::new(position, position + Vec2::new(width as f32, height as f32));
        graphics.draw_rectangle(rect.clone(), *bg_color);
        vga8.draw_sprite_with_color(&rect, 0, (*ch) as u32, *color, graphics);
    }

    pub fn clear_buffer(&mut self) {
        self.display_buffer = [[Tile::new(' '); 80]; 40];
    }

    pub fn display_string(&mut self, str: &str, position: UVec2, color: &Color, bg_color: &Color) {
        let UVec2 { x, y } = position;
        for (i, ch) in str.chars().enumerate() {
            let tile = Tile::new(ch).with_bg(*bg_color).with_fg(*color);
            if x >= 80 || y >= 40 {
                log::warn!("Part of the string is offscreen, no wrapping");
                continue;
            }
            self.display_buffer[y as usize][x as usize + i] = tile;
        }
    }

    pub fn apply_command(&mut self, command: &str) {
		// implement communication protocol here
		
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let width = self.config.grid_width;
        let height = self.config.grid_height;
        for (y, row) in self.display_buffer.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let pos = Vec2::new((x * width as usize) as f32, (y * height as usize) as f32);
                let Tile { ch, fg, bg } = tile;
                self.draw_char(ch, pos, fg, bg, graphics);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    ch: char,
    fg: Color,
    bg: Color,
}

impl Tile {
    pub const fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
    pub const fn with_fg(self, color: Color) -> Self {
        Self { fg: color, ..self }
    }
    pub const fn with_bg(self, color: Color) -> Self {
        Self { bg: color, ..self }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}
