use crate::gui::engine::event::Event;
use hecs::{Entity, World};
use std::sync::{Arc, Mutex};

pub struct Action {
    // must be an option
    // an action needs a mutable borrow to the world to do something
    // but itself is part of the world, so it is not possible to borrow the world as mutable
    // since the action itself is already a borrow to the world
    // thus, the action is taken out of the option, run, and put back in it
    pub action: Option<Arc<Mutex<dyn FnMut(Entity, &mut World) + Send + Sync>>>,
    pub event: Event,
}

impl Action {
    pub fn new<A: 'static + Send + Sync + FnMut(Entity, &mut World)>(a: A, event: Event) -> Self {
        Self {
            action: Some(Arc::new(Mutex::new(a))),
            event,
        }
    }

    pub fn despawn_self_when(event: Event) -> Self {
        Self {
            action: Some(Arc::new(Mutex::new(|my_entity_id, world: &mut World| {
                let _ = world.despawn(my_entity_id);
            }))),
            event,
        }
    }
}
