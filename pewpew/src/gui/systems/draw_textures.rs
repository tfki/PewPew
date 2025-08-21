use crate::gui::components::texture::Texture;
use crate::gui::resources::Resources;
use hecs::World;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn run(canvas: &mut WindowCanvas, world: &mut World, resources: &mut Resources) {
    // draw normal textures
    for (_id, texture) in world.query_mut::<&mut Texture>() {
        let sprite = &resources.images[texture.image_id];
        let tile_size = (
            sprite.query().width,
            sprite.query().height / texture.num_frames,
        );

        // set the current frame by 'scrolling' vertically
        let source_rect = Rect::new(
            0,
            (texture.current_keyframe * tile_size.1) as i32,
            tile_size.0,
            tile_size.1,
        );

        let dest_rect = Rect::new(
            texture.anchor.x,
            texture.anchor.y,
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
    }
}
