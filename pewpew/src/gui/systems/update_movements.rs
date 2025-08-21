use crate::gui::components::movement::Movement;
use crate::gui::components::texture::Texture;
use crate::gui::components::Hitbox;
use hecs::World;
use std::time::SystemTime;

pub fn run(world: &mut World) {
    let now = SystemTime::now();

    // move all hitboxes
    for (_id, (movement, hitbox_opt, texture_opt)) in
        world.query_mut::<(&mut Movement, Option<&mut Hitbox>, Option<&mut Texture>)>()
    {
        if movement.last_movement_time.is_none()
            || (movement.last_movement_time.is_some()
                && now
                    .duration_since(movement.last_movement_time.unwrap())
                    .unwrap()
                    >= movement.every)
        {
            if let Some(hitbox) = hitbox_opt {
                hitbox.anchor = hitbox.anchor + movement.by;
            }
            if let Some(texture) = texture_opt {
                texture.anchor = texture.anchor + movement.by;
            }

            movement.last_movement_time = Some(now);
        }
    }
    // move all textures
}
