use crate::gui::components::hitbox::Hitbox;
use crate::gui::components::movement::Movement;
use crate::gui::components::texture::Texture;
use crate::gui::components::Text;
use crate::gui::stopwatch::Stopwatch;
use hecs::World;

pub fn run(world: &mut World, game_time: &mut Stopwatch) {
    let game_elapsed = game_time.elapsed_ms();

    // move all hitboxes
    for (_id, (movement, hitbox_opt, texture_opt, text_opt)) in world.query_mut::<(
        &mut Movement,
        Option<&mut Hitbox>,
        Option<&mut Texture>,
        Option<&mut Text>,
    )>() {
        if movement.next_movement_at_elapsed_game_time.is_none()
            || (movement.next_movement_at_elapsed_game_time.is_some()
                && game_elapsed >= movement.next_movement_at_elapsed_game_time.unwrap())
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

            movement.next_movement_at_elapsed_game_time = Some(
                movement.every.as_millis() + movement.next_movement_at_elapsed_game_time.unwrap_or(game_elapsed),
            );
        }
    }
    // move all textures
}
