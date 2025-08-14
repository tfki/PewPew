use hecs::World;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use crate::gui::components::{Hitbox, Location};

pub fn run(canvas: &mut WindowCanvas, world: &mut World) {
    for (_id, (hitbox, location)) in world.query_mut::<(&Hitbox, &Location)>() {
        canvas.set_draw_color(Color::WHITE);
        canvas
            .fill_rect(Rect::from_center(
                Point::new(location.x as i32, location.y as i32),
                hitbox.width,
                hitbox.height,
            ))
            .unwrap();
    }
}
