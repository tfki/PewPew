use crate::gui::engine::event::Event;
use hecs::{Entity, World};
use std::sync::{Arc, Mutex};

pub type ActionCallable = dyn FnMut(Entity, &mut World) + Send + Sync;

pub struct Action {
    // must be an option
    // an action needs a mutable borrow to the world to do something
    // but itself is part of the world, so it is not possible to borrow the world as mutable
    // since the action itself is already a borrow to the world
    // thus, the action is taken out of the option, run, and put back in it
    pub action: Option<Arc<Mutex<ActionCallable>>>,
    pub event: Event,
}

impl Action {
    pub fn when<A: 'static + Send + Sync + FnMut(Entity, &mut World)>(event: Event, a: A) -> Self {
        Self {
            action: Some(Arc::new(Mutex::new(a))),
            event,
        }
    }

    pub fn oneshot(mut self) -> Action {
        Action::when_oneshot(self.event, move |entity, world| {
            let action_callable = self.action.take().unwrap();

            (action_callable.lock().unwrap())(entity, world);

            self.action = Some(action_callable);
        })
    }

    pub fn when_oneshot<A: 'static + Send + Sync + FnMut(Entity, &mut World)>(
        event: Event,
        mut a: A,
    ) -> Self {
        let mut has_run = false;
        Action::when(event, move |entity, world| {
            if !has_run {
                a(entity, world);
                has_run = true;
            }
        })
    }

    pub fn despawn_self_when(event: Event) -> Action {
        Action::when_oneshot(event, |my_entity_id, world: &mut World| {
            let _ = world.despawn(my_entity_id);
        })
    }

    pub fn trigger_other_event_when(event: Event, mut other_event: Event) -> Self {
        Action::when(event, move |_, _| other_event.trigger())
    }
}
