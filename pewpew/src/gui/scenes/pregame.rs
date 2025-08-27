use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::{texture, Point, Text};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::load_scenery_textures;
use hecs::{Entity, World};
use log::trace;
use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use crate::gui::engine::components::texture::AnimationEndBehavior;

pub fn run(gui_context: &mut GuiContext) {
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

        resources.images.push(
            texture_creator
                .load_texture(Path::new("./res/flying_huhn.png"))
                .unwrap(),
        );

        load_scenery_textures(&mut resources, &texture_creator);

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

        world.spawn((Text {
            text: "Player 1 missing".to_string(),
            position: PointWithAlignment {
                point: Point { x: 0, y: 0 },
                v_align: VAlign::Top,
                h_align: HAlign::Left,
            },
            scale_numerator: 1,
            scale_denominator: 1,
            original_point: Point { x: 0, y: 0 },
            color: Color::RED,
        },));

        world.spawn((Text {
            text: "Player 2 missing".to_string(),
            position: PointWithAlignment {
                point: Point {
                    x: viewport.width() as i32,
                    y: 0,
                },
                v_align: VAlign::Top,
                h_align: HAlign::Right,
            },
            scale_numerator: 1,
            scale_denominator: 1,
            original_point: Point {
                x: viewport.width() as i32,
                y: 0,
            },
            color: Color::BLUE,
        },));

        world.spawn((Text {
            text: "Player 3 missing".to_string(),
            position: PointWithAlignment {
                point: Point {
                    x: 0,
                    y: viewport.height() as i32,
                },
                v_align: VAlign::Bottom,
                h_align: HAlign::Left,
            },
            scale_numerator: 1,
            scale_denominator: 1,
            original_point: Point {
                x: 0,
                y: viewport.height() as i32,
            },
            color: Color::GREEN,
        },));

        world.spawn((Text {
            text: "Player 4 missing".to_string(),
            position: PointWithAlignment {
                point: Point {
                    x: viewport.width() as i32,
                    y: viewport.height() as i32,
                },
                v_align: VAlign::Bottom,
                h_align: HAlign::Right,
            },
            scale_numerator: 1,
            scale_denominator: 1,
            original_point: Point {
                x: viewport.width() as i32,
                y: viewport.height() as i32,
            },
            color: Color::YELLOW,
        },));

        world.spawn((Text {
            text: "Shoot to join game!".to_string(),
            position: PointWithAlignment::new_center(Point {
                x: viewport.width() as i32 / 2,
                y: viewport.height() as i32 / 2,
            }),
            scale_numerator: 1,
            scale_denominator: 1,
            original_point: Point {
                x: viewport.width() as i32 / 2,
                y: viewport.height() as i32 / 2,
            },
            color: Color::WHITE,
        },));

        // spawn scenery entities
        {
            world.spawn((texture::Builder::new(
                1,
                PointWithAlignment::new_top_left(Point { x: 0, y: 0 }),
            )
            .with_z_index(-10)
            .with_scale(2.0)
            .build(),));

            world.spawn((texture::Builder::new(
                2,
                PointWithAlignment::new_top_left(Point { x: 0, y: 300 }),
            )
            .with_z_index(-9)
            .with_scale(2.0)
            .build(),));

            world.spawn((texture::Builder::new(
                3,
                PointWithAlignment::new_top_left(Point { x: 0, y: 500 }),
            )
            .with_z_index(-8)
            .with_scale(2.0)
            .build(),));

            world.spawn((texture::Builder::new(
                4,
                PointWithAlignment::new_top_left(Point { x: 0, y: 615 }),
            )
            .with_z_index(-7)
            .with_scale(2.0)
            .build(),));
        }

        let mut texture_out_of_viewport_events = spawn_random_chickens(viewport, 150, &mut world);

        game_time.resume();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();
        gui_context.canvas().present();

        for _ in 0..250 {
            let mut new_chickens = Vec::new();
            for (event, entity) in &mut texture_out_of_viewport_events {
                if event.consume() {
                    world.despawn(*entity).unwrap();
                    let mut new_chicken = spawn_random_chickens(viewport, 1, &mut world);
                    new_chickens.append(&mut new_chicken);
                }
            }
            texture_out_of_viewport_events.append(&mut new_chickens);

            let frame_start = SystemTime::now();

            gui_context.canvas().set_draw_color(Color::BLACK);
            gui_context.canvas().clear();

            systems::update_movements::run(&mut world, &mut game_time);
            systems::update_animated_textures::run(&mut world, &mut game_time);
            systems::draw_textures::run(gui_context.canvas(), &mut world, &mut resources);

            gui_context.canvas().set_blend_mode(BlendMode::Blend);
            gui_context
                .canvas()
                .set_draw_color(Color::RGBA(0, 0, 0, 175));
            gui_context
                .canvas()
                .fill_rect(Rect::new(0, 0, 2560, 1440))
                .unwrap();
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

fn spawn_random_chickens(viewport: Rect, n: u32, world: &mut World) -> Vec<(Event, Entity)> {
    let mut texture_out_of_viewport_events = Vec::new();
    for _ in 0..n {
        let out_of_viewport_event = Event::default();
        let rand_big_range = rand::rng().random_range(-5..=5);
        let rand_small_neg_range = rand::rng().random_range(-5..=-1);
        let rand_small_pos_range = rand::rng().random_range(1..=5);

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

        let mut texture_builder = texture::Builder::new(0, position)
            .with_animation_end_behavior(AnimationEndBehavior::Loop)
            .with_scale(rand::rng().random::<f32>() + 0.5)
            .with_num_frames(13)
            .with_frame_advance_interval(Duration::from_millis(rand::rng().random_range(50..150)))
            .on_outside_viewport(out_of_viewport_event.clone());

        if (movement.f)(10000).x > 0 {
            texture_builder = texture_builder.with_vertical_flip();
        }

        let texture = texture_builder.build();
        let entity = world.spawn((movement, texture));
        texture_out_of_viewport_events.push((out_of_viewport_event, entity));
    }

    texture_out_of_viewport_events
}
