use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::texture;
use crate::gui::engine::components::texture::Texture;
use crate::gui::engine::resources::Resources;
use hecs::Bundle;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::path::Path;

#[derive(Bundle)]
pub struct Scenery {
    textures: Vec<Texture>,
}

impl Scenery {
    pub fn new<'a>(
        mut position: PointWithAlignment,
        scale: f32,
        resources: &mut Resources<'a>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let texture_index = resources.images.len();

        // load
        {
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

        // spawn
        let mut textures = Vec::new();
        {
            position.point.y -= (150.0 * scale) as i32;
            textures.push(
                texture::Builder::new(texture_index, position)
                    .with_z_index(-10)
                    .with_scale(scale)
                    .build(),
            );

            position.point.y += (125.0 * scale) as i32;
            textures.push(
                texture::Builder::new(texture_index + 1, position)
                    .with_z_index(-9)
                    .with_scale(scale)
                    .build(),
            );

            position.point.y += (125.0 * scale) as i32;
            textures.push(
                texture::Builder::new(texture_index + 2, position)
                    .with_z_index(-8)
                    .with_scale(scale)
                    .build(),
            );

            position.point.y += (72.5 * scale) as i32;
            textures.push(
                texture::Builder::new(texture_index + 3, position)
                    .with_z_index(-7)
                    .with_scale(scale)
                    .build(),
            );
        }
        Scenery { textures }
    }
}
