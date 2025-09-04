use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::{Point, text, texture, timer};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::common::PlayerData;
use crate::gui::scenes::common::scenery::Scenery;
use crate::gui::scenes::load_all_textures;
use hecs::World;
use log::trace;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::{thread, vec};
use sdl2::render::BlendMode;

pub fn run(gui_context: &mut GuiContext, player_datas: Arc<Mutex<Vec<PlayerData>>>) {
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
        let _texture_id_map = load_all_textures(&mut resources, &texture_creator).unwrap();

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
        let seconds_left = Arc::new(Mutex::new(11));
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
                        .with_color(Color::WHITE)
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

            for _ in 0..250 {
                spawn_new_chicken_event.trigger();
            }
        }

        let num_players = player_datas.lock().unwrap().len();
        for i in 0..num_players {
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

            world.spawn((text::Builder::new(format!("Score: {}", player_datas.lock().unwrap()[i].score), position).with_scale(viewport.height(), 1080).build(),));
        }

        game_time.resume();

        loop {
            if game_end_event.consume_all() > 0 {
                while gui_context.comm().try_recv_from_serial().is_ok() {
                    // empty the buffers
                }
                return;
            }

            let frame_start = SystemTime::now();

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
