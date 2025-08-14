use sdl2::render::Texture;

#[derive(Default)]
pub struct Resources<'a> {
    pub images: Vec<Texture<'a>>
}
