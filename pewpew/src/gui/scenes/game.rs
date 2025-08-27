use crate::gui::engine::components::movement::Movement;
use crate::gui::engine::components::point_with_alignment::{HAlign, PointWithAlignment, VAlign};
use crate::gui::engine::components::texture::{AnimationEndBehavior, Texture};
use crate::gui::engine::components::{texture, timer, Point};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::load_scenery_textures;
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
                .load_texture(Path::new("./res/ammo.png"))
                .unwrap(),
        );

        load_scenery_textures(&mut resources, &texture_creator);

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

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

        let periodical_event = Event::default();
        let timer = timer::Builder::new(Duration::from_secs(3), periodical_event.clone()).build();
        world.spawn((timer,));

        let position = PointWithAlignment {
            point: Point { x: 0, y: 1400 },
            v_align: VAlign::Bottom,
            h_align: HAlign::Left,
        };
        let mut texture_shell_animated = texture::Builder::new(0, position)
            .with_num_frames(21)
            .with_vertical_flip()
            .with_animation_end_behavior(AnimationEndBehavior::Loop)
            .with_scale(2.0)
            .with_frame_advance_interval(Duration::from_millis(66))
            .build();

        let mut texture_shell_without_animation = texture::Builder::new(0, position)
            .with_num_frames(21)
            .with_vertical_flip()
            .with_animation_end_behavior(AnimationEndBehavior::Loop)
            .with_scale(2.0)
            .with_frame_advance_interval(Duration::MAX)
            .build();

        let shell_arch_movement = Movement::new(|t| {
            let t = t as i32;
            Point {
                x: (300.0 * t as f32 / 2000.0) as i32,
                y: (-500.0 + ((1.0 / 64.0) * (t as f32 / 5.0 - 180.0) * (t as f32 / 5.0 - 180.0)))
                    as i32,
            }
        });

        let mut player_1_shell_entities = Vec::new();
        for _ in 0..8 {
            player_1_shell_entities.push(world.spawn((texture_shell_without_animation.clone(),)));
            texture_shell_without_animation.position.point.x += 100;
            texture_shell_without_animation.original_point.x += 100;
        }

        let mut shoot_event = Event::default();
        world.spawn((
            timer::Builder::new(Duration::from_millis(1000), shoot_event.clone())
                .looping()
                .build(),
        ));

        game_time.resume();

        for _frame in 0..500 {
            if shoot_event.consume() {
                if player_1_shell_entities.is_empty() {
                    return;
                }

                let entity = player_1_shell_entities.pop().unwrap();
                {
                    let texture = world.get::<&mut Texture>(entity).unwrap();

                    texture_shell_animated.position = texture.position;
                    texture_shell_animated.original_point = texture.original_point;
                }
                world.spawn((texture_shell_animated.clone(), shell_arch_movement.clone()));

                world.despawn(entity).unwrap();
            }

            let frame_start = SystemTime::now();

            gui_context.canvas().set_draw_color(Color::WHITE);
            gui_context.canvas().clear();

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

            thread::sleep(wait_duration);
        }
    }
}
