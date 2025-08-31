use hecs::World;
use crate::gui::engine::components::condition::Condition;

pub fn run(world: &mut World) {
    for (_, condition) in world
        .query::<&mut Condition>()
        .iter() {

        if (condition.f)(world) {
            condition.event.trigger();
        }
    }
}
