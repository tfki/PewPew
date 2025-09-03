use crate::comm::message::SerialToGuiKind;
use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::texture::Texture;
use crate::gui::engine::components::{Point, hitbox, texture, text};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::common::magazine::Magazine;
use crate::gui::scenes::common::scenery::Scenery;
use crate::gui::scenes::load_all_textures;
use crate::serial::packet::MagazineStatus;
use hecs::World;
use log::trace;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::{thread, vec};

pub fn run(gui_context: &mut GuiContext, sensortag_to_player_id: HashMap<u16, usize>) {
    let viewport = {
        let (width, height) = gui_context.canvas().output_size().unwrap();
        Rect::new(0, 0, width, height)
    };
    let texture_creator = gui_context.canvas().texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();
    {
        let default_font = ttf_context
            .load_font("res/fonts/Walter_Turncoat/WalterTurncoat-Regular.ttf", 128)
            .unwrap();
        let mut resources = Resources::new(default_font);
        let texture_id_map = load_all_textures(&mut resources, &texture_creator).unwrap();

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

        let scenery_scale = viewport.height() as f32 / 720.0;
        world.spawn(Scenery::new(
            PointWithAlignment::new_center(Point {
                x: (viewport.width() / 2) as i32,
                y: (viewport.height() / 2) as i32,
            }),
            scenery_scale,
            &mut resources,
            &texture_creator,
        ));

        // spawn chickens
        {
            // randomly spawns chickens on the edge of the viewport
            // every chicken has an event attached to it that
            // 1. despawns it
            // 2. spawns another random chicken
            // once it has left the viewport
            let mut spawn_new_chicken_event = Event::default();
            world.spawn((Action::spawn_random_chicken_when(
                spawn_new_chicken_event.clone(),
                viewport,
                spawn_new_chicken_event.clone(),
            ),));

            for _ in 0..10 {
                spawn_new_chicken_event.trigger();
            }
        }

        let mut shoot_events = Vec::new();
        let mut reload_events = Vec::new();
        let mut magazine_statuses = Vec::new();
        let mut score_changed_events = Vec::new();

        let ammo_width = resources.images[texture_id_map["ammo.png"]].query().width;
        let magazine_scale = 0.15 * viewport.height() as f32 / ammo_width as f32;
        let scores = Arc::new(Mutex::new(Vec::new()));
        let amount_of_players = sensortag_to_player_id.len();
        // spawn players
        {
            for i in 0..amount_of_players {
                scores.lock().unwrap().push(0);
                let shoot_event = Event::default();
                let mut reload_event = Event::default();
                let score_changed = Event::default();

                let magazine_status = Arc::new(Mutex::new(MagazineStatus {
                    ammo: 8,
                    ammo_max: 8,
                }));
                let position = match i {
                    0 => PointWithAlignment {
                        point: Point { x: 0, y: 0 },
                        v_align: VAlign::Top,
                        h_align: HAlign::Left,
                    },
                    1 => PointWithAlignment {
                        point: Point {
                            x: viewport.width() as i32,
                            y: 0,
                        },
                        v_align: VAlign::Top,
                        h_align: HAlign::Right,
                    },
                    2 => PointWithAlignment {
                        point: Point {
                            x: 0,
                            y: viewport.height() as i32,
                        },
                        v_align: VAlign::Bottom,
                        h_align: HAlign::Left,
                    },
                    3 => PointWithAlignment {
                        point: Point {
                            x: viewport.width() as i32,
                            y: viewport.height() as i32,
                        },
                        v_align: VAlign::Bottom,
                        h_align: HAlign::Right,
                    },
                    _ => unreachable!(),
                };

                // spawn magazine
                // and despawn spawner, so that spawn magazine action only runs once
                let magazine = Magazine::new(
                    shoot_event.clone(),
                    reload_event.clone(),
                    magazine_status.clone(),
                    position,
                    magazine_scale,
                    texture_id_map["ammo.png"],
                    texture_id_map["ammo.png"],
                );
                world.spawn(magazine);

                let score_changed_clone = score_changed.clone();
                let scores_clone = scores.clone();
                world.spawn((vec![Action::when(
                    score_changed.clone(),
                    move |_, world| {
                        let text = text::Builder::new(
                            format!("score: {}", scores_clone.lock().unwrap()[i]),
                            position,
                        )
                        .build();

                        world.spawn((text, Action::despawn_self_when(score_changed_clone.clone())));
                    },
                )],));

                // trigger shot event again, so that the magazine gets a chance to draw itself
                reload_event.trigger();

                shoot_events.push(shoot_event.clone());
                reload_events.push(reload_event.clone());
                magazine_statuses.push(magazine_status.clone());
                score_changed_events.push(score_changed.clone());
            }
        }

        game_time.resume();

        loop {
            let mut is_shooting = false;
            if let Ok(message) = gui_context.comm().try_recv_from_serial() {
                let player_id = sensortag_to_player_id.get(&message.sensortag_id);
                if let Some(player_id) = player_id {
                    match message.kind {
                        SerialToGuiKind::Reload => {
                            *magazine_statuses[*player_id].lock().unwrap() = MagazineStatus {
                                ammo: message.ammo,
                                ammo_max: message.ammo_max,
                            };
                            reload_events[*player_id].trigger();
                        }
                        SerialToGuiKind::Shot => {
                            scores.lock().unwrap()[*player_id] += 1;
                            score_changed_events[*player_id].trigger();

                            *magazine_statuses[*player_id].lock().unwrap() = MagazineStatus {
                                ammo: message.ammo,
                                ammo_max: message.ammo_max,
                            };
                            shoot_events[*player_id].trigger();
                            is_shooting = true;
                        }
                    }
                }
            }

            if is_shooting {
                systems::flashing_sequence::run(gui_context, &mut world, true, &mut game_time);
            }

            let frame_start = SystemTime::now();

            gui_context.canvas().set_draw_color(Color::BLACK);
            gui_context.canvas().clear();

            systems::work_actions::run(&mut world);
            systems::work_timers::run(&mut world, &mut game_time);
            systems::update_movements::run(&mut world, &mut game_time);
            systems::update_animated_textures::run(&mut world, &mut game_time);
            systems::draw_textures::run(gui_context.canvas(), &mut world, &mut resources);
            systems::draw_texts::run(
                gui_context.canvas(),
                &mut world,
                &mut resources,
                &texture_creator,
            );

            gui_context.canvas().present();

            let frame_end = SystemTime::now();
            let frame_duration = frame_end.duration_since(frame_start).unwrap();
            let wait_duration =
                Duration::from_millis(33_u128.saturating_sub(frame_duration.as_millis()) as u64);

            trace!(target: "Gui Thread", "frame took {}ms", frame_duration.as_millis());
            thread::sleep(wait_duration);
        }
    }
}

impl SpawnChickenAction for Action {}
trait SpawnChickenAction {
    fn spawn_random_chicken_when(
        event: Event,
        viewport: Rect,
        out_of_viewport_event: Event,
    ) -> Action {
        Action::when(event, move |_entity_id, world| {
            spawn_random_chickens(viewport, 1, world, out_of_viewport_event.clone());
        })
    }
}

fn spawn_random_chickens(viewport: Rect, n: u32, world: &mut World, out_of_viewport_event: Event) {
    for _ in 0..n {
        let rand_big_range = rand::rng().random_range(-5..=5);
        let rand_small_neg_range = rand::rng().random_range(-5..=-1);
        let rand_small_pos_range = rand::rng().random_range(1..=5);
        let rand_scale = rand::rng().random::<f32>() + 0.5 * (viewport.height() as f32 / 1440.0);

        let (position, movement) = match rand::rng().random_range(0..4) {
            0 => (
                // chicken spawns on top edge
                PointWithAlignment {
                    point: Point {
                        x: rand::rng().random_range(0..viewport.width() as i32),
                        y: 1,
                    },
                    v_align: VAlign::Bottom,
                    h_align: HAlign::Left,
                },
                Movement::new(move |t| Point {
                    x: t as i32 / 33 * rand_big_range,
                    y: t as i32 / 33 * rand_small_pos_range
                        + ((t as f32 / 300.0).sin() * 25.0) as i32,
                }),
            ),
            1 => (
                // chicken spawns on bottom edge
                PointWithAlignment {
                    point: Point {
                        x: rand::rng().random_range(0..viewport.width() as i32),
                        y: viewport.height() as i32 - 1,
                    },
                    v_align: VAlign::Top,
                    h_align: HAlign::Left,
                },
                Movement::new(move |t| Point {
                    x: t as i32 / 33 * rand_big_range,
                    y: t as i32 / 33 * rand_small_neg_range
                        + ((t as f32 / 300.0).sin() * 25.0) as i32,
                }),
            ),
            2 => (
                // chicken spawns on left edge
                PointWithAlignment {
                    point: Point {
                        x: 1,
                        y: rand::rng().random_range(0..viewport.height() as i32),
                    },
                    v_align: VAlign::Top,
                    h_align: HAlign::Right,
                },
                Movement::new(move |t| Point {
                    x: t as i32 / 33 * rand_small_pos_range,
                    y: t as i32 / 33 * rand_big_range + ((t as f32 / 300.0).sin() * 25.0) as i32,
                }),
            ),
            3 => (
                // chicken spawns on right edge
                PointWithAlignment {
                    point: Point {
                        x: viewport.width() as i32 - 1,
                        y: rand::rng().random_range(0..viewport.height() as i32),
                    },
                    v_align: VAlign::Top,
                    h_align: HAlign::Left,
                },
                Movement::new(move |t| Point {
                    x: (t as i32 / 33) * rand_small_neg_range,
                    y: (t as i32 / 33) * rand_big_range + ((t as f32 / 300.0).sin() * 25.0) as i32,
                }),
            ),
            _ => unreachable!(),
        };

        let my_out_of_viewport_event = Event::default();

        let mut texture_builder = texture::Builder::new(0, position)
            .with_scale(rand_scale)
            .with_num_frames(13)
            .with_frame_advance_interval(Duration::from_millis(rand::rng().random_range(50..150)))
            .on_outside_viewport(my_out_of_viewport_event.clone());

        if (movement.f)(10000).x > 0 {
            texture_builder = texture_builder.with_vertical_flip();
        }

        let hit_event = Event::default();
        let texture = texture_builder.build();
        let out_of_viewport_event_clone = out_of_viewport_event.clone();
        world.spawn((
            movement,
            texture,
            hitbox::Builder::new(
                position,
                (200.0 * rand_scale) as u32,
                (200.0 * rand_scale) as u32,
            )
            .on_hit(hit_event.clone())
            .build(),
            vec![
                Action::despawn_self_when(my_out_of_viewport_event.clone()),
                Action::trigger_other_event_when(
                    my_out_of_viewport_event,
                    out_of_viewport_event.clone(),
                ),
                Action::when(hit_event, move |entity_id, world| {
                    {
                        let out_of_frame_event = Event::default();
                        let dying_texture = {
                            let entity = world.entity(entity_id).unwrap();
                            let old_texture = entity.get::<&Texture>().unwrap();

                            texture::Builder::new(2, old_texture.position)
                                .with_scale(old_texture.scale)
                                .with_num_frames(8)
                                .with_frame_advance_interval(Duration::from_millis(50))
                                .on_outside_viewport(out_of_frame_event.clone())
                                .build()
                        };
                        let movement = Movement {
                            f: Arc::new(move |t| Point {
                                x: 0,
                                y: ((t as f32 / 2500.0) * viewport.height() as f32).powi(1) as i32,
                            }),
                            first_invocation_game_time: None,
                        };
                        world.spawn((
                            movement,
                            dying_texture,
                            vec![
                                Action::despawn_self_when(out_of_frame_event.clone()),
                                Action::spawn_random_chicken_when(
                                    out_of_frame_event,
                                    viewport,
                                    out_of_viewport_event_clone.clone(),
                                ),
                            ],
                        ));
                    }
                    let _ = world.despawn(entity_id);
                }),
            ],
        ));
    }
}
