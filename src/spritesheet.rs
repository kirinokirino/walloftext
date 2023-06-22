use speedy2d::{color::Color, image::ImageHandle, Graphics2D};

use glam::Vec2;
use glam_rect::Rect;

pub struct Spritesheet {
    image_handle: ImageHandle,
    pub width: u32,
    pub height: u32,
}

impl Spritesheet {
    pub fn new(image_handle: ImageHandle, width: u32, height: u32) -> Self {
        let image_size = image_handle.size();
        let (sprite_width, sprite_height) = (image_size.x / width, image_size.y / height);
        if sprite_width * width != image_size.x {
            todo!("Image size division ");
        }
        if sprite_height * height != image_size.y {
            todo!("Image size division ");
        }
        Self {
            image_handle,
            width,
            height,
        }
    }
    pub fn draw_sprite_with_color(
        &self,
        dest: &Rect,
        sprite_x: u32,
        sprite_y: u32,
        color: Color,
        graphics: &mut Graphics2D,
    ) {
        draw_sprite(
            dest,
            &self.image_handle,
            sprite_x,
            sprite_y,
            self.width,
            self.height,
            Some(color),
            graphics,
        );
    }

    pub fn draw_sprite(
        &self,
        dest: &Rect,
        sprite_x: u32,
        sprite_y: u32,
        graphics: &mut Graphics2D,
    ) {
        draw_sprite(
            dest,
            &self.image_handle,
            sprite_x,
            sprite_y,
            self.width,
            self.height,
            None,
            graphics,
        );
    }
}

pub fn draw_sprite(
    destination: &Rect,
    spritesheet: &ImageHandle,
    sprite_x: u32,
    sprite_y: u32,
    spritesheet_width: u32,
    spritesheet_height: u32,
    color: Option<Color>,
    graphics: &mut Graphics2D,
) {
    let vertex_positions_clockwise = [
        destination.top_left,
        destination.top_right(),
        destination.bottom_right,
        destination.bottom_left(),
    ];
    let image_coords_normalized = [
        Vec2::new(
            sprite_x as f32 / spritesheet_width as f32,
            sprite_y as f32 / spritesheet_height as f32,
        ),
        Vec2::new(
            (sprite_x + 1) as f32 / spritesheet_width as f32,
            sprite_y as f32 / spritesheet_height as f32,
        ),
        Vec2::new(
            (sprite_x + 1) as f32 / spritesheet_width as f32,
            (sprite_y + 1) as f32 / spritesheet_height as f32,
        ),
        Vec2::new(
            sprite_x as f32 / spritesheet_width as f32,
            (sprite_y + 1) as f32 / spritesheet_height as f32,
        ),
    ];
    let vertex_colors = color.map_or(
        [Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE],
        |color| [color, color, color, color],
    );
    graphics.draw_quad_image_tinted_four_color(
        vertex_positions_clockwise,
        vertex_colors,
        image_coords_normalized,
        spritesheet,
    );
}
