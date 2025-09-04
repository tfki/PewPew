use crate::gui::engine::resources::Resources;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;

pub mod common;
pub mod game;
pub mod intro;
pub mod pregame;
pub mod sandbox;
pub mod scoreboard;

#[derive(Debug)]
pub enum LoadAllTexturesError {
    DuplicateName,
    FileNotFound,
}
pub fn load_all_textures<'a>(
    resources: &mut Resources<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<HashMap<String, usize>, LoadAllTexturesError> {
    let mut file_idx_map = HashMap::new();

    let files_to_load = [Path::new("res/images/flying_huhn.png"), Path::new("res/images/ammo.png"), Path::new("res/images/flying_huhn_dying.png")];

    for (idx, path) in files_to_load.iter().enumerate() {
        //                           -------------------------wat?--------------------------
        if file_idx_map
            .insert(path.file_name().unwrap().to_str().unwrap().to_string(), idx)
            .is_some()
        {
            return Err(LoadAllTexturesError::DuplicateName);
        }

        resources.images.push(
            texture_creator
                .load_texture(path)
                .map_err(|_| LoadAllTexturesError::FileNotFound)?,
        );
    }

    Ok(file_idx_map)
}
