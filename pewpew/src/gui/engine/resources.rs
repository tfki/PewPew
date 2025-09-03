use crate::gui::engine::components::text::Text;
use sdl2::render::Texture;
use sdl2::ttf::Font;
use std::collections::HashMap;
use std::time::Instant;

pub struct Resources<'sdl_ctx> {
    pub images: Vec<Texture<'sdl_ctx>>,
    pub default_font: Font<'sdl_ctx, 'static>,
    pub text_cache: HashMap<(String, u32, u32), (Instant, Texture<'sdl_ctx>)>,
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
        let key = (
            text.text.clone(),
            text.scale_numerator,
            text.scale_denominator,
        );
        if !self.text_cache.contains_key(&key) {
            if self.text_cache.len() > 500 {
                let rm_key = if let Some((key, _)) = self
                    .text_cache
                    .iter()
                    .min_by(|(_, (t1, _)), (_, (t2, _))| t1.cmp(t2))
                {
                    Some(key.clone())
                } else {
                    None
                };

                if let Some(key) = rm_key {
                    self.text_cache.remove(&key);
                }
            }

            self.text_cache
                .insert(key.clone(), (Instant::now(), create_texture(self)));
        }
        &self.text_cache[&key].1
    }
}
