use crate::gui::components::movement::Movement;
use crate::gui::components::texture::Texture;
use crate::gui::components::{Hitbox, Text};
use hecs::World;
use std::time::SystemTime;

pub fn run(world: &mut World) {
    let now = SystemTime::now();

    // move all hitboxes
    for (_id, (movement, hitbox_opt, texture_opt, text_opt)) in
        world.query_mut::<(&mut Movement, Option<&mut Hitbox>, Option<&mut Texture>, Option<&mut Text>)>()
    {
        if movement.last_movement_time.is_none()
            || (movement.last_movement_time.is_some()
                && now
                    .duration_since(movement.last_movement_time.unwrap())
                    .unwrap()
                    >= movement.every)
        {
            if let Some(hitbox) = hitbox_opt {
                hitbox.position.point = hitbox.position.point + movement.by;
            }
            if let Some(texture) = texture_opt {
                texture.position.point = texture.position.point + movement.by;
            }
            if let Some(text) = text_opt {
                text.position.point = text.position.point + movement.by;
            }

            movement.last_movement_time = Some(now);
        }
    }
    // move all textures
}
