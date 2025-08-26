use sdl2::render::Texture;
use sdl2::ttf::{Font};

pub struct Resources<'a, 'b> {
    pub images: Vec<Texture<'a>>,
    pub default_font: Font<'a, 'b>,
}

impl<'a, 'b> Resources<'a, 'b> {
    pub fn new(default_font: Font<'a, 'b>) -> Self {
        Resources {
            images: vec![],
            default_font
        }
    }
}
