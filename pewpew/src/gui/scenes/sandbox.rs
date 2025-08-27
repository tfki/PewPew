use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::texture::{AnimationEndBehavior, Texture};
use crate::gui::engine::components::{hitbox, texture, timer, Point, Text};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use hecs::World;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn run(gui_context: &mut GuiContext) {
    let texture_creator = gui_context.canvas().texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();
    {
        let default_font = ttf_context
            .load_font("res/fonts/Walter_Turncoat/WalterTurncoat-Regular.ttf", 128)
            .unwrap();
        let mut resources = Resources::new(default_font);

        resources.images.push(
            texture_creator
                .load_texture(Path::new("./res/intro_huhn_in_hole.png"))
                .unwrap(),
        ); // https://onlinetools.com/image/remove-specific-color-from-image

        resources.images.push(
            texture_creator
                .load_texture(Path::new("./res/flying_huhn.png"))
                .unwrap(),
        ); // https://onlinetools.com/image/remove-specific-color-from-image

        resources.images.push(
            texture_creator
                .load_texture(Path::new("./res/flying_huhn_dying.png"))
                .unwrap(),
        ); // https://onlinetools.com/image/remove-specific-color-from-image

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

        let mut chickens_with_events = Vec::new();

        let periodical_event = Event::default();
        let timer = timer::Builder::new(Duration::from_secs(3), periodical_event.clone()).build();
        world.spawn((timer,));

        for i in 1..10 {
            world.spawn((
                Text {
                    text: "Text works hehe".to_string(),
                    position: PointWithAlignment::new_center(Point { x: 0, y: 100 * i }),
                    original_point: Point { x: 0, y: 100 * i },
                    scale_numerator: i as u32,
                    scale_denominator: 10,
                    color: Color::RGB(255, 0, (i * 25) as u8),
                },
                Movement::new(move |t| Point {
                    x: i * t as i32 / 33,
                    y: 0,
                }),
            ));
        }

        for x in 0..5 {
            for y in 0..5 {
                let position = PointWithAlignment::new_top_left(Point {
                    x: x * 200,
                    y: y * 200,
                });
                let texture = texture::Builder::new(1, position)
                    .with_num_frames(13)
                    .with_vertical_flip()
                    .with_animation_end_behavior(AnimationEndBehavior::Loop)
                    .with_frame_advance_interval(Duration::from_millis(
                        (10 * (10 - (x + y))) as u64,
                    ))
                    .build();

                let movement = Movement::new(move |t| Point {
                    x: t as i32 / 33 * (x + y),
                    y: 0,
                });

                let event = Event::default();
                let entity = world.spawn((
                    texture,
                    hitbox::Builder::new(position, 200, 200)
                        .on_hit(event.clone())
                        .build(),
                    movement,
                ));

                chickens_with_events.push((entity, event));
            }
        }

        game_time.resume();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();
        gui_context.canvas().present();

        {
            for frame in 0..500 {
                for (entity, event) in &mut chickens_with_events {
                    if event.consume() {
                        let death_texture = {
                            let texture = world.get::<(&Texture)>(*entity).unwrap();

                            texture::Builder::new(2, texture.position)
                                .with_num_frames(8)
                                .with_frame_advance_interval(Duration::from_millis(110))
                                .with_animation_end_behavior(AnimationEndBehavior::Freeze)
                                .build()
                        };
                        let movement = Movement::new(move |t| {
                            Point{ x: 0, y: (t as f32 / 64.0).powi(2) as i32 }
                        });

                        world.spawn((death_texture, movement));
                        world.despawn(*entity).unwrap();
                    }
                }

                let frame_start = SystemTime::now();

                gui_context.canvas().set_draw_color(Color::BLACK);
                gui_context.canvas().clear();

                if frame % 100 == 10 {
                    // then, show flashing sequence
                    // this takes how many ever frames it needs O(ceil(log2(|hitboxes|)))
                    systems::flashing_sequence::run(gui_context, &mut world, true, &mut game_time);
                } else {
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
                }

                gui_context.canvas().present();

                let frame_end = SystemTime::now();
                let frame_duration = frame_end.duration_since(frame_start).unwrap();
                let wait_duration = Duration::from_millis(
                    33_u128.saturating_sub(frame_duration.as_millis()) as u64,
                );

                thread::sleep(wait_duration);
            }
        }
    }
}
