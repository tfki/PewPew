use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment};
use crate::gui::engine::components::texture::Texture;
use crate::gui::engine::components::{texture, Point};
use crate::gui::engine::event::Event;
use crate::serial::packet::MagazineStatus;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(hecs::Bundle)]
pub struct Magazine {
    actions: Vec<Action>,
}

impl Magazine {
    pub fn new(
        shoot_event: Event,
        reload_event: Event,
        magazine_status: Arc<Mutex<MagazineStatus>>,
        position: PointWithAlignment,
        scale: f32,
        ammo_virgin_texture_id: usize,
        ammo_used_texture_id: usize,
    ) -> Magazine {
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

        Magazine {
            actions: Action::update_ammo_info_when(
                shoot_event,
                reload_event,
                magazine_status,
                position,
                scale,
                shell_used_texture,
                shell_virgin_texture,
                shell_arch_movement,
            ),
        }
    }
}

impl MakeAmmoWork for Action {}
pub trait MakeAmmoWork {
    #[allow(clippy::too_many_arguments)]
    fn update_ammo_info_when(
        shoot_event: Event,
        reload_event: Event,
        magazine_status: Arc<Mutex<MagazineStatus>>,
        start_position: PointWithAlignment,
        scale: f32,
        ammo_used_texture: Texture,
        ammo_virgin_texture: Texture,
        shot_movement: Movement,
    ) -> Vec<Action> {
        let shell_entities = Arc::new(Mutex::new(Vec::new()));
        let shell_entities_clone1 = shell_entities.clone();
        let shell_entities_clone2 = shell_entities.clone();
        let mut ammo_virgin_texture_clone1 = ammo_virgin_texture.clone();
        let mut ammo_used_texture_clone1 = ammo_used_texture.clone();
        let mut ammo_virgin_texture_clone2 = ammo_virgin_texture.clone();
        let magazine_status_clone = magazine_status.clone();
        vec![Action::when(shoot_event, move |_, world| {
            let magazine_status = magazine_status_clone.lock().unwrap();

            for entity in shell_entities_clone1.lock().unwrap().iter() {
                let _ = world.despawn(*entity);
            }

            ammo_virgin_texture_clone1.position = match start_position.h_align {
                HAlign::Left => start_position,
                HAlign::Center => {
                    let mut pos = start_position;
                    pos.point.x -= (magazine_status.ammo_max as i32 * (50.0 * scale) as i32) / 2;
                    pos
                }
                HAlign::Right => {
                    let mut pos = start_position;
                    pos.point.x -= magazine_status.ammo_max as i32 * (50.0 * scale) as i32;
                    pos
                }
            };

            // do one more and turn it into a used shell
            for _ in 0..=magazine_status.ammo {
                shell_entities_clone1.lock().unwrap().push(world.spawn((ammo_virgin_texture_clone1.clone(),)));

                ammo_virgin_texture_clone1.position.point.x += (50.0 * scale) as i32;
            }
            {
                let last_shell_entity_id = shell_entities_clone1.lock().unwrap().pop().unwrap();
                let last_shell_entity = world
                    .get::<&mut Texture>(last_shell_entity_id)
                    .unwrap()
                    .clone();

                ammo_used_texture_clone1.position = last_shell_entity.position;
                ammo_used_texture_clone1.original_point = last_shell_entity.position.point;
                world.despawn(last_shell_entity_id).unwrap();
            }

            let shell_gone_event = Event::default();
            ammo_used_texture_clone1.outside_viewport_event = Some(shell_gone_event.clone());
            world.spawn((
                ammo_used_texture_clone1.clone(),
                shot_movement.clone(),
                Action::despawn_self_when(shell_gone_event),
            ));
        }), Action::when(reload_event, move |_, world| {
            let magazine_status = magazine_status.lock().unwrap();

            for entity in shell_entities_clone2.lock().unwrap().iter() {
                let _ = world.despawn(*entity);
            }

            ammo_virgin_texture_clone2.position = match start_position.h_align {
                HAlign::Left => start_position,
                HAlign::Center => {
                    let mut pos = start_position;
                    pos.point.x -= (magazine_status.ammo_max as i32 * (50.0 * scale) as i32) / 2;
                    pos
                }
                HAlign::Right => {
                    let mut pos = start_position;
                    pos.point.x -= magazine_status.ammo_max as i32 * (50.0 * scale) as i32;
                    pos
                }
            };

            for _ in 0..magazine_status.ammo {
                shell_entities_clone2.lock().unwrap().push(world.spawn((ammo_virgin_texture_clone2.clone(),)));

                ammo_virgin_texture_clone2.position.point.x += (50.0 * scale) as i32;
            }
        })]
    }
}


pub trait SpawnMagazineAction {
    fn spawn_magazine_when(
        shoot_event: Event,
        reload_event: Event,
        magazine_status: Arc<Mutex<MagazineStatus>>,
        position: PointWithAlignment,
        scale: f32,
        ammo_virgin_texture_id: usize,
        ammo_used_texture_id: usize,
    ) -> Action {
        let mut shoot_event_clone = shoot_event.clone();
        let reload_event_clone = reload_event.clone();
        let magazine_status_clone = magazine_status.clone();
        Action::when_oneshot(shoot_event, move |_, world| {
            let magazine = Magazine::new(
                shoot_event_clone.clone(),
                reload_event_clone.clone(),
                magazine_status_clone.clone(),
                position,
                scale,
                ammo_virgin_texture_id,
                ammo_used_texture_id,
            );
            world.spawn(magazine);

            // trigger shot event again, so that the magazine gets a chance to draw itself
            shoot_event_clone.trigger();
        })
    }
}

impl SpawnMagazineAction for Action {}
