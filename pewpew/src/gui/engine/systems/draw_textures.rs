use crate::gui::engine::components::texture::Texture;
use crate::gui::engine::resources::Resources;
use hecs::{Entity, World};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn run(canvas: &mut WindowCanvas, world: &mut World, resources: &mut Resources) {
    let mut textures: Vec<(Entity, &mut Texture)> =
        world.query_mut::<&mut Texture>().into_iter().collect();

    // textures with lower z_index must be drawn first
    textures.sort_by(|(_, texture1), (_, texture2)| texture1.z_index.cmp(&texture2.z_index));

    let viewport_rect = {
        let (x, y) = canvas.output_size().unwrap();
        Rect::new(0, 0, x, y)
    };

    // draw normal textures
    for (_id, texture) in textures {
        let sprite = &resources.images[texture.image_id];
        let query = sprite.query();
        let tile_size = (query.width, query.height / texture.num_frames);

        // set the current frame by 'scrolling' vertically
        let source_rect = Rect::new(
            0,
            (texture.current_keyframe * tile_size.1) as i32,
            tile_size.0,
            tile_size.1,
        );

        let dest_rect = texture.position.align_rect(
            (tile_size.0 as f32 * texture.scale) as u32,
            (tile_size.1 as f32 * texture.scale) as u32,
        );

        // trigger texture position events
        let viewport_intersect = dest_rect.intersection(viewport_rect);
        if let Some(viewport_intersect) = viewport_intersect {
            if viewport_intersect != dest_rect
                && let Some(event) = &mut texture.at_viewport_edge_event
            {
                event.trigger();
            }
        } else if let Some(event) = &mut texture.outside_viewport_event {
            event.trigger();
        }

        canvas
            .copy_ex(
                sprite,
                Some(source_rect),
                Some(dest_rect),
                texture.rotation_deg,
                None,
                texture.flip_horizontally,
                texture.flip_vertically,
            )
            .unwrap();
    }
}
