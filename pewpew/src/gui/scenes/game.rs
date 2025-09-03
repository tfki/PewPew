use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::Point;
use crate::gui::engine::gui_context::GuiContext;
use crate::gui::engine::resources::Resources;
use crate::gui::engine::stopwatch::Stopwatch;
use crate::gui::engine::systems;
use crate::gui::scenes::common::scenery::Scenery;
use hecs::World;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn run(gui_context: &mut GuiContext) {
    // let viewport = {
    //     let (width, height) = gui_context.canvas().output_size().unwrap();
    //     Rect::new(0, 0, width, height)
    // };
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

        let mut world = World::new();
        world.spawn(Scenery::new(
            PointWithAlignment::new_top_left(Point { x: 0, y: 0 }),
            2.0,
            &mut resources,
            &texture_creator,
        ));

        let mut game_time = Stopwatch::new_paused();

        // let magazine = Magazine::new(
        //     Event::default(),
        //     Event::default(),
        //     Arc::new(Mutex::new(MagazineStatus{ ammo: 0, ammo_max: 0 })),
        //     PointWithAlignment {
        //         point: Point {
        //             x: 0,
        //             y: viewport.height() as i32,
        //         },
        //         h_align: HAlign::Left,
        //         v_align: VAlign::Bottom,
        //     },
        //     2.0,
        //     0,
        //     1,
        // );
        // world.spawn(magazine);
        // world.spawn((timer::Builder::new(Duration::from_secs(1), shoot_event)
        //     .looping()
        //     .build(),));

        game_time.resume();

        for _ in 0..200 {
            let frame_start = SystemTime::now();

            gui_context.canvas().set_draw_color(Color::BLACK);
            gui_context.canvas().clear();

            systems::work_timers::run(&mut world, &mut game_time);
            systems::work_actions::run(&mut world);
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
                Duration::from_millis(16_u128.saturating_sub(frame_duration.as_millis()) as u64);

            thread::sleep(wait_duration);
        }
    }
}
