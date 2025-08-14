use sdl2::render::Texture;

pub struct Resources<'a> {
    pub images: Vec<Texture<'a>>
}

impl<'a> Default for Resources<'a> {
    fn default() -> Self {
        Resources { images: vec![] }
    }
}
