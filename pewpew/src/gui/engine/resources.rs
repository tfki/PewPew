use crate::gui::engine::components::Text;
use sdl2::render::Texture;
use sdl2::ttf::Font;
use std::collections::HashMap;

pub struct Resources<'sdl_ctx> {
    pub images: Vec<Texture<'sdl_ctx>>,
    pub default_font: Font<'sdl_ctx, 'static>,
    pub text_cache: HashMap<Text, Texture<'sdl_ctx>>,
}

impl<'sdl_ctx> Resources<'sdl_ctx> {
    pub fn new(default_font: Font<'sdl_ctx, 'static>) -> Self {
        Resources {
            images: vec![],
            default_font,
            text_cache: HashMap::new(),
        }
    }

    pub fn get_cached_or_render_text_with<T: FnOnce(&Resources) -> Texture<'sdl_ctx>>(
        &mut self,
        text: &Text,
        create_texture: T,
    ) -> &Texture<'sdl_ctx> {
        if !self.text_cache.contains_key(text) {
            self.text_cache.insert(text.clone(), create_texture(self));
        }
        &self.text_cache[text]
    }
}
