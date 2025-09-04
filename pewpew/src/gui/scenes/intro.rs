use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::{Point, text, texture};
use crate::gui::engine::event::Event;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use hecs::World;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use sdl2::mixer::Chunk;

pub fn run(gui_context: &mut GuiContext) {
    let viewport = {
        let (width, height) = gui_context.canvas().output_size().unwrap();
        Rect::new(0, 0, width, height)
    };
    let texture_creator = gui_context.canvas().texture_creator();
    let ttf_context = sdl2::ttf::init().unwrap();

    let intro = Chunk::from_file("res/audio/valve_intro.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&intro, 0).unwrap();
    {
        let default_font = ttf_context
            .load_font("res/fonts/Walter_Turncoat/WalterTurncoat-Regular.ttf", 128)
            .unwrap();
        let mut resources = Resources::new(default_font);

        resources.images.push(
            texture_creator
                .load_texture(Path::new("res/images/intro_huhn_in_hole.png"))
                .unwrap(),
        ); // https://onlinetools.com/image/remove-specific-color-from-image

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

        let position = PointWithAlignment::new_center(Point {
            x: (viewport.width() / 2) as i32,
            y: (viewport.height() / 2) as i32,
        });
        let mut intro_done_event = Event::default();
        let texture = texture::Builder::new(0, position)
            .with_num_frames(14)
            .with_vertical_flip()
            .with_scale(viewport.height() as f32 / 360.0)
            .with_frame_advance_interval(Duration::from_millis(600))
            .on_animation_end(intro_done_event.clone())
            .build();

        world.spawn((texture,));

        world.spawn((text::Builder::new(
            "Moorhuhn".to_string(),
            PointWithAlignment::new_center(Point {
                x: (viewport.width() / 2) as i32,
                y: (2 * viewport.height() / 3) as i32,
            }),
        )
        .with_scale(viewport.height(), 1440)
        .build(),));

        game_time.resume();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();
        gui_context.canvas().present();

        {
            loop {
                if intro_done_event.consume_all() > 0 {
                    break;
                }

                let frame_start = SystemTime::now();

                gui_context.canvas().set_draw_color(Color::BLACK);
                gui_context.canvas().clear();

                systems::work_timers::run(&mut world, &mut game_time);
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
                let wait_duration = Duration::from_millis(
                    33_u128.saturating_sub(frame_duration.as_millis()) as u64,
                );

                thread::sleep(wait_duration);
            }
        }
    }
}
