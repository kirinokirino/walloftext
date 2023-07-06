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

    cursor: Cursor,
    commands: Vec<Command>,

    viewport_size: UVec2,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        let buffer = [[Tile::new(' '); 80]; 40];
        let cursor = Cursor::new('a', Color::WHITE, Color::BLACK, 0, 0);
        Self {
            config,
            images: Vec::new(),
            spritesheets: Vec::new(),
            counter: 0,
            display_buffer: buffer,

            cursor,
            commands: Vec::new(),

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

        for command in std::mem::take(&mut self.commands).into_iter() {
            match command {
                Command::Write => self.display_cursor(),
                Command::Up => {
                    self.cursor.y = if self.cursor.y > 0 {
                        self.cursor.y - 1
                    } else {
                        0
                    }
                }
                Command::Down => {
                    self.cursor.y = if self.cursor.y < 40 {
                        self.cursor.y + 1
                    } else {
                        40
                    }
                }
                Command::Left => {
                    self.cursor.x = if self.cursor.x > 0 {
                        self.cursor.x - 1
                    } else {
                        0
                    }
                }
                Command::Right => {
                    self.cursor.x = if self.cursor.x < 80 {
                        self.cursor.x + 1
                    } else {
                        80
                    }
                }
                other => (),
            }
        }
    }

    fn display_cursor(&mut self) {
        let Cursor {
            character,
            foreground,
            background,
            x,
            y,
        } = self.cursor;

        self.display_string(
            &character.to_string(),
            UVec2::new(x.into(), y.into()),
            &foreground,
            &background,
        );
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
        let commands = command.split('-');
        for command in commands {
            let words = command.split(' ');
            for word in words {
                self.apply_word(word);
            }
        }
    }

    pub fn apply_word(&mut self, word: &str) {
        match word {
            "w" => self.commands.push(Command::Write),
            "u" => self.commands.push(Command::Up),
            "d" => self.commands.push(Command::Down),
            "l" => self.commands.push(Command::Left),
            "r" => self.commands.push(Command::Right),
            other => self.display_string(word, UVec2::new(0, 40), &Color::BLUE, &Color::WHITE),
        }
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

#[derive(Clone, Copy, Debug)]
enum Command {
    Write,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Cursor {
    character: char,
    foreground: Color,
    background: Color,
    x: u8,
    y: u8,
}

impl Cursor {
    pub const fn new(character: char, foreground: Color, background: Color, x: u8, y: u8) -> Self {
        Self {
            character,
            foreground,
            background,
            x,
            y,
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
