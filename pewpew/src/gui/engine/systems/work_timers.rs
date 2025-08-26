use hecs::World;
use crate::gui::engine::components::timer::Timer;
use crate::gui::engine::stopwatch::Stopwatch;

pub fn run(world: &mut World, game_time: &mut Stopwatch) {
    let game_elapsed = game_time.elapsed_ms();
    let mut to_delete = Vec::new();
    let mut actions_to_run = Vec::new();

    for (id, timer) in world.query_mut::<&mut Timer>() {
        if timer.next_activation_at_elapsed_game_time.is_none()
            || (timer.next_activation_at_elapsed_game_time.is_some()
                && game_elapsed >= timer.next_activation_at_elapsed_game_time.unwrap())
        {
            timer.next_activation_at_elapsed_game_time = Some(
                timer
                    .next_activation_at_elapsed_game_time
                    .unwrap_or(game_elapsed)
                    + timer.duration.as_millis(),
            );

            actions_to_run.push((id, timer.action.clone()));
            if !timer.looping {
                to_delete.push(id);
            }
        }
    }

    for (entity, action) in actions_to_run {
        action(entity, world);
    }

    for entity in to_delete {
        let _ = world.despawn(entity);
    }
}
