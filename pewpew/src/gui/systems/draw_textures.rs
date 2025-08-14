use crate::gui::components::texture::Texture;
use hecs::World;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::time::SystemTime;
use crate::gui::components::Location;
use crate::gui::resources::Resources;

pub fn run(canvas: &mut WindowCanvas, world: &mut World, resources: &mut Resources) {
    let now = SystemTime::now();
    let mut to_be_despawned = Vec::new();

    // draw normal textures
    for (id, (texture, location)) in world.query_mut::<(&mut Texture, &Location)>() {
        let sprite = &resources.images[texture.image_id];
        let tile_size = (
            sprite.query().width,
            sprite.query().height / texture.num_frames,
        );

        // set the current frame by 'scrolling' vertically
        let source_rect = Rect::new(
            0,
            (texture.current_frame * tile_size.1) as i32,
            tile_size.0,
            tile_size.1,
        );

        let dest_rect = Rect::from_center(
            Point::new(location.x as i32, location.y as i32),
            (tile_size.0 as f32 * texture.scale) as u32,
            (tile_size.1 as f32 * texture.scale) as u32,
        );

        canvas
            .copy_ex(
                &resources.images[texture.image_id],
                Some(source_rect),
                Some(dest_rect),
                texture.rotation_deg,
                None,
                false,
                false,
            )
            .unwrap();

        match texture.last_frame_time {
            None => texture.last_frame_time = Some(now),
            Some(last_frame_time)
                if now.duration_since(last_frame_time).unwrap()
                    >= texture.frame_advance_interval =>
            {
                texture.current_frame += 1;
                texture.last_frame_time = Some(now);

                if texture.repeat {
                    texture.current_frame %= texture.num_frames;
                } else if !texture.repeat && texture.current_frame == texture.num_frames {
                    to_be_despawned.push(id);
                }
            }
            _ => {}
        }
    }

    for id in to_be_despawned {
        world.despawn(id).unwrap();
    }
}
