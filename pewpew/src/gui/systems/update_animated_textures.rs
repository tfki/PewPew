use crate::gui::components::texture::Texture;
use crate::gui::stopwatch::Stopwatch;
use hecs::World;

pub fn run(world: &mut World, game_time: &mut Stopwatch) {
    let game_elapsed = game_time.elapsed_ms();
    let mut to_be_despawned = Vec::new();

    // draw normal textures
    for (id, texture) in world.query_mut::<&mut Texture>() {
        match texture.next_keyframe_switch_at_elapsed_game_time {
            Some(next_keyframe_switch_time) if next_keyframe_switch_time <= game_elapsed => {
                texture.current_keyframe += 1;
                texture.next_keyframe_switch_at_elapsed_game_time =
                    Some(next_keyframe_switch_time + texture.keyframe_duration.as_millis());

                if texture.repeat {
                    texture.current_keyframe %= texture.num_frames;
                } else if !texture.repeat && texture.current_keyframe == texture.num_frames {
                    to_be_despawned.push(id);
                }
            }
            None => {
                texture.next_keyframe_switch_at_elapsed_game_time =
                    Some(game_elapsed + texture.keyframe_duration.as_millis())
            }
            _ => {}
        }
    }

    for id in to_be_despawned {
        world.despawn(id).unwrap();
    }
}
