use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator};
use sdl2::video::WindowContext;
use crate::gui::engine::resources::Resources;

pub mod intro;
pub mod pregame;
pub mod game;
pub mod sandbox;

pub fn load_scenery_textures<'a>(resources: &mut Resources<'a>, texture_creator: &'a TextureCreator<WindowContext>) {
    resources.images.push(
        texture_creator
            .load_texture(Path::new("./res/sky.png"))
            .unwrap(),
    );

    resources.images.push(
        texture_creator
            .load_texture(Path::new("./res/backdrop.png"))
            .unwrap(),
    );

    resources.images.push(
        texture_creator
            .load_texture(Path::new("./res/castle.png"))
            .unwrap(),
    );

    resources.images.push(
        texture_creator
            .load_texture(Path::new("./res/foreground.png"))
            .unwrap(),
    );

    resources.images.push(
        texture_creator
            .load_texture(Path::new("./res/tree.png"))
            .unwrap(),
    );
}
