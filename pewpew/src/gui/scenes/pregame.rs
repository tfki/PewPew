use crate::comm::message::SerialToGuiKind;
use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::{Point, text, texture};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::common::PlayerData;
use crate::gui::scenes::common::magazine::SpawnMagazineAction;
use crate::gui::scenes::common::scenery::Scenery;
use crate::gui::scenes::load_all_textures;
use crate::serial::packet::MagazineStatus;
use hecs::World;
use log::trace;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sdl2::mixer::Chunk;

pub fn run(gui_context: &mut GuiContext) -> Arc<Mutex<Vec<PlayerData>>> {
    let viewport = {
        let (width, height) = gui_context.canvas().output_size().unwrap();
        Rect::new(0, 0, width, height)
    };
    let texture_creator = gui_context.canvas().texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();

    let shoot_sounds = [Chunk::from_file("res/../res/audio/gun-shot-359196.mp3").unwrap(), Chunk::from_file("res/../res/audio/glock19-18535.mp3").unwrap()];
    let reload_sounds = [Chunk::from_file("res/../res/audio/ak47_boltpull.mp3").unwrap(), Chunk::from_file("res/../res/audio/_en_sound_glock18-slideforward_.mp3").unwrap()];
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

            for _ in 0..250 {
                spawn_new_chicken_event.trigger();
            }
        }

        let player_datas = Arc::new(Mutex::new(Vec::new()));
        let mut shoot_events = Vec::new();
        let mut reload_events = Vec::new();
        let player_names = ["Player 1", "Player 2", "Player 3", "Player 4"];
        let player_colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];

        let all_players_joined_event = Event::default();
        let some_player_joined_event = Event::default();
        let countdown_tick_event = Event::default();
        let start_countdown_event = Event::default();
        let mut countdown_finished_event = Event::default();

        let num_players = Arc::new(Mutex::new(0));
        let countdown_seconds_left = Arc::new(Mutex::new(COUNTDOWN_START_VALUE as i32));

        const COUNTDOWN_START_VALUE: u8 = 15;
        let ammo_width = resources.images[texture_id_map["ammo.png"]].query().width;
        let magazine_scale = 0.15 * viewport.height() as f32 / ammo_width as f32;

        // spawn players
        {
            for i in 0..2 {
                let shoot_event = Event::default();
                let reload_event = Event::default();

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

                // create and despawn "player x missing" text
                world.spawn((
                    text::Builder::new(
                        format!("{} missing", player_names[i]).to_string(),
                        position,
                    )
                        .with_color(player_colors[i])
                        .with_scale(viewport.height(), 1440)
                        .build(),
                    Action::despawn_self_when(shoot_event.clone()),
                ));

                // spawn magazine
                // and despawn spawner, so that spawn magazine action only runs once
                world.spawn((vec![
                    Action::spawn_magazine_when(
                        shoot_event.clone(),
                        reload_event.clone(),
                        player_datas.clone(),
                        i,
                        position,
                        magazine_scale,
                        texture_id_map["ammo.png"],
                        texture_id_map["ammo.png"],
                    ),
                    Action::despawn_self_when(shoot_event.clone()),
                ],));

                shoot_events.push(shoot_event.clone());
                reload_events.push(reload_event.clone());
            }
        }

        world.spawn((
            text::Builder::new(
                "Shoot to join game!".to_string(),
                PointWithAlignment::new_center(Point {
                    x: viewport.width() as i32 / 2,
                    y: viewport.height() as i32 / 2,
                }),
            )
                .with_scale(viewport.height(), 1440)
                .build(),
            Action::despawn_self_when(all_players_joined_event.clone()),
        ));

        for shoot_event in &mut shoot_events {
            let num_players_clone = num_players.clone();
            let all_players_joined_event_clone = all_players_joined_event.clone();
            world.spawn((vec![
                Action::trigger_other_event_when(
                    shoot_event.clone(),
                    some_player_joined_event.clone(),
                )
                    .oneshot(),
                Action::despawn_self_when(shoot_event.clone()),
                Action::when_oneshot(shoot_event.clone(), move |_, _| {
                    let mut lock = num_players_clone.lock().unwrap();
                    *lock += 1;
                    if *lock == 4 {
                        all_players_joined_event_clone.clone().trigger();
                    }
                }),
            ],));
        }

        // manages countdown text
        {
            let start_game_countdown_tick_event = countdown_tick_event.clone();
            let countdown_seconds_left = countdown_seconds_left.clone();
            let mut countdown_finished_event = countdown_finished_event.clone();

            // when countdown_tick_event is triggered
            // add text with countdown_seconds_left that disappears on the next tick of
            // countdown_tick_event and decrement countdown_seconds_left
            world.spawn((vec![Action::when(
                countdown_tick_event.clone(),
                move |_, world| {
                    world.spawn((
                        text::Builder::new(
                            format!(
                                "Game starts in {}..",
                                countdown_seconds_left.lock().unwrap()
                            )
                                .to_string(),
                            PointWithAlignment {
                                point: Point {
                                    x: (viewport.width() / 2) as i32,
                                    y: (2 * viewport.height() / 3) as i32,
                                },
                                v_align: VAlign::Center,
                                h_align: HAlign::Center,
                            },
                        )
                            .with_color(Color::WHITE)
                            .with_scale(viewport.height(), 1440)
                            .build(),
                        Action::despawn_self_when(start_game_countdown_tick_event.clone()),
                    ));

                    let mut locked = countdown_seconds_left.lock().unwrap();
                    *locked -= 1;
                    if *locked == -1 {
                        countdown_finished_event.trigger();
                    }
                },
            )],));
        }

        // start_countdown_event trigger starts timer on countdown_tick_event
        // another start_countdown_event trigger restarts the timer
        {
            // when start_countdown_event is triggered,
            // install a timer that triggers countdown_tick_event every second
            // and that despawns itself when start_countdown_event is triggered once more
            let mut countdown_tick_event = countdown_tick_event.clone();
            let start_countdown_event = start_countdown_event.clone();
            world.spawn((vec![Action::when(
                start_countdown_event.clone(),
                move |_, world| {
                    world.spawn((
                        countdown_tick_event
                            .clone()
                            .trigger_every(Duration::from_secs(1)),
                        Action::despawn_self_when(start_countdown_event.clone()),
                    ));
                    countdown_tick_event.trigger();
                },
            )],));
        }

        // reset countdown to initial value when a player joins
        {
            let countdown_value_clone = countdown_seconds_left.clone();
            world.spawn((Action::when(
                some_player_joined_event.clone(),
                move |_, _| {
                    *countdown_value_clone.lock().unwrap() = COUNTDOWN_START_VALUE as i32;
                },
            ),));
        }

        // start countdown when a player joins (oneshot)
        // and install same action again
        // this must be oneshot to prevent the edge case where multiple players join within the same frame
        // and the countdown start event is then fired multiple times
        // that is not possible in this constellation
        world.spawn((vec![
            Action::trigger_other_event_when(
                some_player_joined_event.clone(),
                start_countdown_event.clone(),
            )
                .oneshot(),
            Action::despawn_self_when(some_player_joined_event.clone()),
            Action::when(some_player_joined_event.clone(), move |_entity, world| {
                world.spawn((vec![
                    Action::trigger_other_event_when(
                        some_player_joined_event.clone(),
                        start_countdown_event.clone(),
                    )
                        .oneshot(),
                    Action::despawn_self_when(some_player_joined_event.clone()),
                ],));
            }),
        ],));

        game_time.resume();

        loop {
            if countdown_finished_event.consume_all() > 0 {
                return player_datas;
            }

            if let Ok(message) = gui_context.comm().try_recv_from_serial() {
                let player_id = {
                    let mut locked = player_datas.lock().unwrap();
                    if let Some((idx, _)) = locked
                        .iter()
                        .enumerate()
                        .find(|(_, data)| data.sensortag_id == message.sensortag_id) {
                        idx
                    } else {
                        let new_player_id = locked.len();
                        locked.push(PlayerData {
                            sensortag_id: message.sensortag_id,
                            magazine_status: MagazineStatus {
                                ammo: message.ammo,
                                ammo_max: message.ammo_max,
                            },
                            score: 0,
                        });
                        new_player_id
                    }
                };

                match message.kind {
                    SerialToGuiKind::Reload => {
                        sdl2::mixer::Channel::all().play(&reload_sounds[player_id], 0).unwrap();

                        player_datas.lock().unwrap()[player_id].magazine_status = MagazineStatus {
                            ammo: message.ammo,
                            ammo_max: message.ammo_max,
                        };
                        reload_events[player_id].trigger();
                    }
                    SerialToGuiKind::Shot => {
                        player_datas.lock().unwrap()[player_id].magazine_status = MagazineStatus {
                            ammo: message.ammo,
                            ammo_max: message.ammo_max,
                        };

                        if message.ammo > 0 {
                            sdl2::mixer::Channel::all().play(&shoot_sounds[player_id], 0).unwrap();
                            shoot_events[player_id].trigger();
                        } else  {
                            sdl2::mixer::Channel::all().play(&dry_shot_sound, 0).unwrap();
                        }
                    }
                }
            }

            let frame_start = Instant::now();

            gui_context.canvas().set_draw_color(Color::BLACK);
            gui_context.canvas().clear();

            systems::work_actions::run(&mut world);
            systems::work_timers::run(&mut world, &mut game_time);
            systems::update_movements::run(&mut world, &mut game_time);
            systems::update_animated_textures::run(&mut world, &mut game_time);
            systems::draw_textures::run(gui_context.canvas(), &mut world, &mut resources);

            // make everything drawn up to this point appear slightly darker
            gui_context.canvas().set_blend_mode(BlendMode::Blend);
            gui_context
                .canvas()
                .set_draw_color(Color::RGBA(0, 0, 0, 150));
            gui_context.canvas().fill_rect(viewport).unwrap();
            gui_context.canvas().set_blend_mode(BlendMode::None);

            systems::draw_texts::run(
                gui_context.canvas(),
                &mut world,
                &mut resources,
                &texture_creator,
            );

            gui_context.canvas().present();

            let frame_end = Instant::now();
            let frame_duration = frame_end.duration_since(frame_start);
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

        let texture = texture_builder.build();
        world.spawn((
            movement,
            texture,
            vec![
                Action::despawn_self_when(my_out_of_viewport_event.clone()),
                Action::trigger_other_event_when(
                    my_out_of_viewport_event,
                    out_of_viewport_event.clone(),
                ),
            ],
        ));
    }
}
