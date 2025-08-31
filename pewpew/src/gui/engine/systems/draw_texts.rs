use crate::gui::engine::resources::Resources;
use hecs::World;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use crate::gui::engine::components::text::Text;

pub fn run<'a>(
    canvas: &mut WindowCanvas,
    world: &mut World,
    resources: &mut Resources<'a>,
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
) {
    for (_id, text) in world.query_mut::<&mut Text>() {
        // apparently rendering text is expensive af, so we cache rendered text
        let texture = resources.get_cached_or_render_text_with(text, |resources: &Resources| {
            let surface = resources
                .default_font
                .render(text.text.as_str())
                .blended(text.color)
                .unwrap();
            texture_creator
                .create_texture_from_surface(&surface)
                .unwrap()
        });

        let TextureQuery { width, height, .. } = texture.query();
        let scaled_width = (width as f32 * text.scale()) as u32;
        let scaled_height = (height as f32 * text.scale()) as u32;

        canvas
            .copy(
                texture,
                None,
                text.position.align_rect(scaled_width, scaled_height),
            )
            .unwrap();
    }
}
