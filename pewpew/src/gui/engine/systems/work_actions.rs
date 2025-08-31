use hecs::World;
use crate::gui::engine::components::action::Action;

pub fn run(world: &mut World) {
    // collect ALL actions first
    // in case an action despawns another entity with an action
    // i want both actions to run if their event triggered

    // collect single-action-entities
    let mut entities_with_action = world
        .query::<&mut Action>()
        .iter()
        .filter_map(
            |(entity, wrapped_action)| match wrapped_action.event.consume() {
                true => Some((entity, wrapped_action.action.take().unwrap())),
                false => None,
            },
        )
        .collect::<Vec<_>>();

    // collect multi-action-entities
    let entities_with_action_vec = world
        .query::<&mut Vec<Action>>()
        .iter()
        .flat_map(|(entity, vec)| {
            vec.iter_mut().enumerate().filter_map(
                move |(idx, wrapped_action)| match wrapped_action.event.consume() {
                    true => Some((entity, idx, wrapped_action.action.take().unwrap())),
                    false => None,
                },
            )
        })
        .collect::<Vec<_>>();

    // handle all entities with a single action
    for (entity_id, mut action) in entities_with_action {
        {
            let mut locked_action = action.lock().unwrap();
            locked_action(entity_id, world);
        }

        // use if let, in case the action despawned the entity
        if let Ok(entity) = world.entity(entity_id) {
            entity.get::<&mut Action>().unwrap().action = Some(action);
        }
    }

    // handle all entities with a vector of actions
    for (entity_id, idx, mut action) in entities_with_action_vec {
        {
            let mut locked_action = action.lock().unwrap();
            locked_action(entity_id, world);
        }

        // use if let, in case the action despawned the entity
        if let Ok(entity) = world.entity(entity_id) {
            entity.get::<&mut Vec<Action>>().unwrap()[idx].action = Some(action);
        }
    }
}
