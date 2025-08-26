use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use hecs::{Entity, World};
use log::debug;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use crate::gui::engine::components::{movement, texture, timer, Point, Text};
use crate::gui::engine::components::hitbox::Hitbox;
use crate::gui::engine::components::movement::By;
use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;

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

        let printer = |_: Entity, _: &mut World| {
            debug!(
                "hello from timer hehe {}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
        };
        world.spawn((timer::Builder::new(Duration::from_secs(3), printer)
                         .looping()
                         .build(),));

        for i in 1..10 {
            world.spawn((
                Text {
                    text: "Text works hehe".to_string(),
                    position: PointWithAlignment::new_center(Point { x: 0, y: 100 * i }),
                    scale: i as f32 * 1.0 / 10.0,
                    color: Color::RGB(255, 0, (i * 25) as u8),
                },
                movement::Builder::new(By { x: i, y: 0 }, Duration::from_millis(33)).build(),
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
                    .looping()
                    .with_frame_advance_interval(Duration::from_millis(
                        (10 * (10 - (x + y))) as u64,
                    ))
                    .build();

                let movement =
                    movement::Builder::new(By { x: x + y, y: 0 }, Duration::from_millis(33))
                        .build();

                world.spawn((
                    texture,
                    Hitbox {
                        position,
                        width: 200,
                        height: 200,
                        z_index: 0,
                    },
                    movement,
                ));
            }
        }

        game_time.resume();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();
        gui_context.canvas().present();

        {
            for frame in 0..500 {
                let frame_start = SystemTime::now();

                gui_context.canvas().set_draw_color(Color::BLACK);
                gui_context.canvas().clear();

                if frame % 100 == 0 {
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
