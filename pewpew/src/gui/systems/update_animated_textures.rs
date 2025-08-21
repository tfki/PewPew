use crate::gui::components::texture::Texture;
use hecs::World;
use std::time::SystemTime;

pub fn run(world: &mut World) {
    let now = SystemTime::now();
    let mut to_be_despawned = Vec::new();

    // draw normal textures
    for (id, texture) in world.query_mut::<&mut Texture>() {
        match texture.last_keyframe_change_time {
            None => texture.last_keyframe_change_time = Some(now),
            Some(last_update_time)
            if now.duration_since(last_update_time).unwrap()
                >= texture.keyframe_duration =>
                {
                    texture.current_keyframe += 1;
                    texture.last_keyframe_change_time = Some(now);

                    if texture.repeat {
                        texture.current_keyframe %= texture.num_frames;
                    } else if !texture.repeat && texture.current_keyframe == texture.num_frames {
                        to_be_despawned.push(id);
                    }
                }
            _ => {}
        }
    }

    for id in to_be_despawned {
        world.despawn(id).unwrap();
    }
}
