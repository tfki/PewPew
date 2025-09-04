use crate::comm::message::SerialToGuiKind;
use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::hitbox::Hitbox;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::texture::{AnimationEndBehavior, Texture};
use crate::gui::engine::components::{Point, hitbox, text, texture, timer};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::common::PlayerData;
use crate::gui::scenes::common::magazine::Magazine;
use crate::gui::scenes::common::scenery::Scenery;
use crate::gui::scenes::load_all_textures;
use crate::serial::packet::MagazineStatus;
use hecs::World;
use log::trace;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::{thread, vec};
use sdl2::mixer::Chunk;

const GAME_DURATION_SEC: u64 = 20;

pub fn run(gui_context: &mut GuiContext, player_datas: Arc<Mutex<Vec<PlayerData>>>) -> Arc<Mutex<Vec<PlayerData>>> {
    let viewport = {
        let (width, height) = gui_context.canvas().output_size().unwrap();
        Rect::new(0, 0, width, height)
    };
    let texture_creator = gui_context.canvas().texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();

    // Load and play mp3
    let shoot_sounds = [Chunk::from_file("res/audio/gun-shot-359196.mp3").unwrap(), Chunk::from_file("res/audio/glock19-18535.mp3").unwrap()];
    let reload_sounds = [Chunk::from_file("res/audio/ak47_boltpull.mp3").unwrap(), Chunk::from_file("res/audio/_en_sound_glock18-slideforward_.mp3").unwrap()];
    let death_sounds = [Chunk::from_file("res/audio/wilhelm_scream.mp3").unwrap(), Chunk::from_file("res/audio/ahhhh.mp3").unwrap()];
    let dry_shot_sound = Chunk::from_file("res/../res/audio/dry-fire-364846.mp3").unwrap();
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

        // game end event
        let mut game_end_event = Event::default();
        let seconds_left = Arc::new(Mutex::new(GAME_DURATION_SEC));
        let mut game_countdown_tick = Event::default();

        world.spawn((timer::Builder::new(
            Duration::from_secs(*seconds_left.lock().unwrap()),
            game_end_event.clone(),
        )

        .build(),));
        world.spawn((
            timer::Builder::new(Duration::from_secs(1), game_countdown_tick.clone())
                .looping()
                .build(),
        ));

        let game_countdown_tick_clone = game_countdown_tick.clone();
        world.spawn((Action::when(
            game_countdown_tick_clone.clone(),
            move |_, world| {
                *seconds_left.lock().unwrap() -= 1;
                world.spawn((
                    text::Builder::new(
                        format!("{}", seconds_left.lock().unwrap()),
                        PointWithAlignment {
                            point: Point {
                                x: (viewport.width() / 2) as i32,
                                y: 0,
                            },
                            v_align: VAlign::Top,
                            h_align: HAlign::Center,
                        },
                    )
                        .with_color(Color::BLACK)
                        .with_scale(viewport.height(), 1080)
                    .build(),
                    Action::despawn_self_when(game_countdown_tick_clone.clone()),
                ));
            },
        ),));
        game_countdown_tick.trigger();

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
        let mut score_changed_events = Vec::new();

        let ammo_width = resources.images[texture_id_map["ammo.png"]].query().width;
        let magazine_scale = 0.15 * viewport.height() as f32 / ammo_width as f32;
        let amount_of_players = player_datas.lock().unwrap().len();
        // spawn players
        {
            for i in 0..amount_of_players {
                let shoot_event = Event::default();
                let mut reload_event = Event::default();
                let mut score_changed = Event::default();

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
                    player_datas.clone(),
                    i,
                    position,
                    magazine_scale,
                    texture_id_map["ammo.png"],
                    texture_id_map["ammo.png"],
                );
                world.spawn(magazine);

                let mut score_position = position;
                match i {
                    0 | 1 => {
                        score_position.point.y +=
                            ((viewport.height() as f32 / 1080.0) * 150.0) as i32
                    }
                    2 | 3 => {
                        score_position.point.y -=
                            ((viewport.height() as f32 / 1080.0) * 150.0) as i32
                    }
                    _ => unreachable!(),
                }
                let score_changed_clone = score_changed.clone();
                let player_datas_clone = player_datas.clone();
                world.spawn((vec![Action::when(
                    score_changed.clone(),
                    move |_, world| {
                        let text = text::Builder::new(
                            format!("score: {}", player_datas_clone.lock().unwrap()[i].score),
                            score_position,
                        )
                        .with_color(Color::BLACK)
                        .with_scale(viewport.height(), 2160)
                        .build();

                        world.spawn((text, Action::despawn_self_when(score_changed_clone.clone())));
                    },
                )],));

                // trigger shot event again, so that the magazine gets a chance to draw itself
                reload_event.trigger();

                // trigger once so score is visible at the beginning
                score_changed.trigger();

                shoot_events.push(shoot_event.clone());
                reload_events.push(reload_event.clone());
                score_changed_events.push(score_changed.clone());
            }
        }

        game_time.resume();

        loop {
            if game_end_event.consume_all() > 0 {
                return player_datas;
            }

            let mut shooter = None;
            if let Ok(message) = gui_context.comm().try_recv_from_serial() {
                let mut lock = player_datas.lock().unwrap();
                let player_id = lock
                    .iter_mut()
                    .enumerate()
                    .find(|(_, data)| data.sensortag_id == message.sensortag_id);
                if let Some((player_id, data)) = player_id {
                    match message.kind {
                        SerialToGuiKind::Reload => {
                            sdl2::mixer::Channel::all().play(&reload_sounds[player_id], 0).unwrap();

                            data.magazine_status = MagazineStatus {
                                ammo: message.ammo,
                                ammo_max: message.ammo_max,
                            };
                            reload_events[player_id].trigger();
                        }
                        SerialToGuiKind::Shot => {
                            let is_dry_shot = message.ammo == 0 && lock[player_id].magazine_status.ammo == 0;

                            lock[player_id].magazine_status = MagazineStatus {
                                ammo: message.ammo,
                                ammo_max: message.ammo_max,
                            };

                            if is_dry_shot {
                                sdl2::mixer::Channel::all().play(&dry_shot_sound, 0).unwrap();
                            } else  {
                                sdl2::mixer::Channel::all().play(&shoot_sounds[player_id], 0).unwrap();

                                shooter = Some((player_id, message.sensortag_id));
                            }

                            shoot_events[player_id].trigger();
                        }
                    }
                }
            }

            if let Some((player_id, sensortag_id)) = shooter {
                if let Some(victim_id) = systems::flashing_sequence::run(
                    gui_context,
                    &mut world,
                    true,
                    &mut game_time,
                    sensortag_id,
                ) {
                    sdl2::mixer::Channel::all().play(&death_sounds[player_id], 0).unwrap();

                    let victim = world.entity(victim_id).unwrap();
                    let hitbox = victim.get::<&Hitbox>().unwrap();

                    let score =
                        (hitbox.width as f32 / 200.0) / (viewport.height() as f32 / 1440.0) - 0.5;

                    player_datas.lock().unwrap()[player_id].score +=
                        20_u32.saturating_sub((score * 5.0) as u32);
                    score_changed_events[player_id].trigger();
                }

                while let Ok(message) = gui_context.comm().try_recv_from_serial() {
                    let mut lock = player_datas.lock().unwrap();
                    let player_id = lock
                        .iter_mut()
                        .enumerate()
                        .find(|(_, data)| data.sensortag_id == message.sensortag_id);
                    if let Some((player_id, data)) = player_id {
                        match message.kind {
                            SerialToGuiKind::Reload => {
                                sdl2::mixer::Channel::all().play(&reload_sounds[player_id], 0).unwrap();

                                data.magazine_status = MagazineStatus {
                                    ammo: message.ammo,
                                    ammo_max: message.ammo_max,
                                };
                                reload_events[player_id].trigger();
                            }
                            SerialToGuiKind::Shot => {
                                let mut locked = player_datas.lock().unwrap();
                                let is_dry_shot = message.ammo == 0 && locked[player_id].magazine_status.ammo == 0;

                                locked[player_id].magazine_status = MagazineStatus {
                                    ammo: message.ammo,
                                    ammo_max: message.ammo_max,
                                };

                                if is_dry_shot {
                                    sdl2::mixer::Channel::all().play(&dry_shot_sound, 0).unwrap();
                                } else  {
                                    sdl2::mixer::Channel::all().play(&shoot_sounds[player_id], 0).unwrap();
                                }

                                shoot_events[player_id].trigger();
                            }
                        }
                    }
                }
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
                                .with_animation_end_behavior(AnimationEndBehavior::Freeze)
                                .with_frame_advance_interval(Duration::from_millis(99))
                                .on_outside_viewport(out_of_frame_event.clone())
                                .build()
                        };
                        let movement = Movement {
                            f: Arc::new(move |t| Point {
                                x: 0,
                                y: ((t as f32 / 1500.0).powi(2) * viewport.height() as f32) as i32,
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
