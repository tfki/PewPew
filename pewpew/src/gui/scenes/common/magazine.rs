use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::texture::Texture;
use crate::gui::engine::components::{texture, Point};
use crate::gui::engine::event::Event;
use std::time::Duration;

#[derive(hecs::Bundle)]
pub struct Magazine {
    action: Action,
}

impl Magazine {
    pub fn new_w_event(
        position: PointWithAlignment,
        num_shells: usize,
        scale: f32,
        ammo_virgin_texture_id: usize,
        ammo_used_texture_id: usize,
        shoot_event: Event,
    ) -> (Magazine, Event) {
        let shell_used_texture = texture::Builder::new(ammo_used_texture_id, position)
            .with_num_frames(21)
            .with_vertical_flip()
            .with_scale(scale)
            .with_z_index(10)
            .with_frame_advance_interval(Duration::from_millis(66))
            .build();

        let shell_virgin_texture = texture::Builder::new(ammo_virgin_texture_id, position)
            .with_num_frames(21)
            .with_vertical_flip()
            .with_scale(scale)
            .with_z_index(10)
            .with_frame_advance_interval(Duration::MAX)
            .build();

        let shell_arch_movement = Movement::new(move |t| {
            let t = t as i32;
            Point {
                x: (300.0 * t as f32 / 2000.0) as i32,
                y: (-500.0 + ((1.0 / 64.0) * (t as f32 / 5.0 - 180.0).powi(2))) as i32,
            }
        });

        (
            Magazine {
                action: Action::update_ammo_info_when(
                    shoot_event.clone(),
                    position,
                    num_shells,
                    scale,
                    shell_used_texture,
                    shell_virgin_texture,
                    shell_arch_movement,
                ),
            },
            shoot_event,
        )
    }

    pub fn new(
        position: PointWithAlignment,
        num_shells: usize,
        scale: f32,
        ammo_virgin_texture_id: usize,
        ammo_used_texture_id: usize,
    ) -> (Magazine, Event) {
        let shoot_event = Event::default();
        Self::new_w_event(
            position,
            num_shells,
            scale,
            ammo_virgin_texture_id,
            ammo_used_texture_id,
            shoot_event,
        )
    }
}

impl MakeAmmoWork for Action {}
pub trait MakeAmmoWork {
    fn update_ammo_info_when(
        event: Event,
        start_position: PointWithAlignment,
        num_shells: usize,
        scale: f32,
        mut ammo_used_texture: Texture,
        mut ammo_virgin_texture: Texture,
        shot_movement: Movement,
    ) -> Action {
        let mut shell_entities = Vec::new();
        Action::new(
            move |entity, world| {
                if shell_entities.is_empty() {
                    // reload

                    ammo_virgin_texture.position = start_position;
                    for _ in 0..=num_shells {
                        shell_entities.push(world.spawn((ammo_virgin_texture.clone(),)));

                        ammo_virgin_texture.position.point.x += (50.0 * scale) as i32;
                    }
                } else {
                    {
                        let last_shell_entity_id = shell_entities.pop().unwrap();
                        let last_shell_entity = world
                            .get::<&mut Texture>(last_shell_entity_id)
                            .unwrap()
                            .clone();

                        ammo_used_texture.position = last_shell_entity.position;
                        ammo_used_texture.original_point = last_shell_entity.position.point;
                        world.despawn(last_shell_entity_id).unwrap();
                    }

                    let shell_gone_event = Event::default();
                    ammo_used_texture.outside_viewport_event = Some(shell_gone_event.clone());
                    world.spawn((
                        ammo_used_texture.clone(),
                        shot_movement.clone(),
                        Action::despawn_self_when(shell_gone_event),
                    ));
                }
            },
            event,
        )
    }
}
