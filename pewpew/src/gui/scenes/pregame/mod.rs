use crate::gui::engine::components::action::Action;
use crate::gui::engine::components::condition::Condition;
use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::{text, texture, timer, Point};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::common::magazine::Magazine;
use crate::gui::scenes::common::scenery::Scenery;
use crate::gui::scenes::load_all_textures;
use hecs::{Bundle, Entity, World};
use log::trace;
use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

mod custom_components;
mod custom_systems;

pub trait SpawnMagazineAction {
    fn spawn_magazine_when(
        mut shoot_event: Event,
        position: PointWithAlignment,
        num_shells: usize,
        scale: f32,
        ammo_virgin_texture_id: usize,
        ammo_used_texture_id: usize,
    ) -> Action {
        let mut shoot_event_clone = shoot_event.clone();
        Action::new(
            move |_, world| {
                let (magazine, _) = Magazine::new_w_event(
                    position,
                    num_shells,
                    scale,
                    ammo_virgin_texture_id,
                    ammo_used_texture_id,
                    shoot_event_clone.clone(),
                );
                world.spawn(magazine);

                // trigger shot event again, so that the magazine gets a chance to draw itself
                shoot_event_clone.trigger();
            },
            shoot_event,
        )
    }
}

impl SpawnMagazineAction for Action {}

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
        let texture_id_map = load_all_textures(&mut resources, &texture_creator).unwrap();

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

        world.spawn(Scenery::new(
            PointWithAlignment::new_top_left(Point { x: 0, y: 0 }),
            2.0,
            &mut resources,
            &texture_creator,
        ));

        let p3_shot = Event::default();
        {
            // on p3_shot trigger
            let position = PointWithAlignment {
                point: Point {
                    x: 0,
                    y: viewport.height() as i32,
                },
                v_align: VAlign::Bottom,
                h_align: HAlign::Left,
            };

            // despawn "p1 missing" text
            world.spawn((
                text::Builder::new("Player 3 missing".to_string(), position)
                    .with_color(Color::RED)
                    .build(),
                Action::despawn_self_when(p3_shot.clone()),
            ));

            // spawn magazine
            // and despawn spawner, so that spawn magazine action only runs once
            world.spawn((vec![
                Action::spawn_magazine_when(
                    p3_shot.clone(),
                    position,
                    8,
                    2.0,
                    texture_id_map["ammo.png"],
                    texture_id_map["ammo.png"],
                ),
                Action::despawn_self_when(p3_shot.clone()),
            ],));
        }

        world.spawn((p3_shot.trigger_every(Duration::from_secs(3)),));

        world.spawn((text::Builder::new(
            "Shoot to join game!".to_string(),
            PointWithAlignment::new_center(Point {
                x: viewport.width() as i32 / 2,
                y: viewport.height() as i32 / 2,
            }),
        )
        .build(),));

        // randomly spawns chickens on the edge of the viewport
        // every chicken has an event attached to it that
        // 1. despawns it
        // 2. spawns another random chicken
        // once it has left the viewport
        spawn_random_chickens(viewport, 250, &mut world);

        let condition = Condition::new(|world| true, Event::default());
        world.spawn((condition,));

        game_time.resume();

        for _ in 0..200 {
            let frame_start = SystemTime::now();

            gui_context.canvas().set_draw_color(Color::BLACK);
            gui_context.canvas().clear();

            systems::work_actions::run(&mut world);
            systems::work_timers::run(&mut world, &mut game_time);
            systems::update_movements::run(&mut world, &mut game_time);
            systems::update_animated_textures::run(&mut world, &mut game_time);
            systems::draw_textures::run(gui_context.canvas(), &mut world, &mut resources);

            // make everything drawn up to this point apper slightly darker
            gui_context.canvas().set_blend_mode(BlendMode::Blend);
            gui_context
                .canvas()
                .set_draw_color(Color::RGBA(0, 0, 0, 175));
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

impl ReplaceOutOfViewportChickenAction for Action {}
trait ReplaceOutOfViewportChickenAction {
    fn replace_out_of_viewport_chicken_when(event: Event, viewport: Rect) -> Action {
        Action::new(
            move |entity_id, world| {
                world.despawn(entity_id).unwrap();
                spawn_random_chickens(viewport, 1, world);
            },
            event,
        )
    }
}

fn spawn_random_chickens(viewport: Rect, n: u32, world: &mut World) {
    for _ in 0..n {
        let out_of_viewport_event = Event::default();
        let rand_big_range = rand::rng().random_range(-5..=5);
        let rand_small_neg_range = rand::rng().random_range(-5..=-1);
        let rand_small_pos_range = rand::rng().random_range(1..=5);
        let rand_scale = rand::rng().random::<f32>() + 0.5;

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
            .with_scale(rand_scale)
            .with_num_frames(13)
            .with_frame_advance_interval(Duration::from_millis(rand::rng().random_range(50..150)))
            .on_outside_viewport(out_of_viewport_event.clone());

        if (movement.f)(10000).x > 0 {
            texture_builder = texture_builder.with_vertical_flip();
        }

        let texture = texture_builder.build();
        world.spawn((
            movement,
            texture,
            Action::replace_out_of_viewport_chicken_when(out_of_viewport_event, viewport),
        ));
    }
}
