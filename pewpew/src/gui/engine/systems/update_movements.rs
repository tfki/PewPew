use crate::gui::engine::components::hitbox::Hitbox;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::texture::Texture;
use crate::gui::engine::components::Text;
use crate::gui::engine::stopwatch::Stopwatch;
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
        if movement.first_invocation_game_time.is_none() {
            movement.first_invocation_game_time = Some(game_elapsed);
        }

        let t = game_elapsed - movement.first_invocation_game_time.unwrap();

        let diff = (movement.f)(t);

        if let Some(hitbox) = hitbox_opt {
            hitbox.position.point = hitbox.original_point + diff;
        }
        if let Some(texture) = texture_opt {
            texture.position.point = texture.original_point + diff;
        }
        if let Some(text) = text_opt {
            text.position.point = text.original_point + diff;
        }
    }
    // move all textures
}
