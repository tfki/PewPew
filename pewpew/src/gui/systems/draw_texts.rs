use crate::gui::resources::Resources;
use hecs::World;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use crate::gui::components::Text;

pub fn run(canvas: &mut WindowCanvas, world: &mut World, resources: &mut Resources, texture_creator: &TextureCreator<sdl2::video::WindowContext>) {
    // draw normal textures
    for (_id, text) in world.query_mut::<&mut Text>() {
        // render a surface, and convert it to a texture bound to the canvas
        let surface = resources.default_font
            .render(text.text.as_str())
            .blended(text.color)
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let scaled_width = (width as f32 * text.scale) as u32;
        let scaled_height = (height as f32 * text.scale) as u32;

        canvas.copy(&texture, None, text.position.align_rect(scaled_width, scaled_height)).unwrap();
    }
}
