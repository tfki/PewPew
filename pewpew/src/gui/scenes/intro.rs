use crate::gui::engine::components::point_with_alignment::PointWithAlignment;
use crate::gui::engine::components::{texture, Point, Text};
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
use crate::gui::engine::event::Event;

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

        let mut world = World::new();
        let mut game_time = Stopwatch::new_paused();

        let position = PointWithAlignment::new_center(Point {
            x: 2560 / 2,
            y: 1440 / 2,
        });
        let mut intro_done_event = Event::default();
        let texture = texture::Builder::new(0, position)
            .with_num_frames(14)
            .with_vertical_flip()
            .looping()
            .with_scale(4.0)
            .with_frame_advance_interval(Duration::from_millis(200))
            .on_animation_end(intro_done_event.clone())
            .build();

        world.spawn((texture,));

        world.spawn((Text{
            text: "Moorhuhn".to_string(),
            position: PointWithAlignment::new_center(Point { x: 2560/2, y: 2*1440/3 }),
            scale_numerator: 1,
            scale_denominator: 1,
            original_point: Point { x: 2560/2, y: 2*1440/3 },
            color: Color::YELLOW,
        },));

        game_time.resume();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();
        gui_context.canvas().present();

        {
            loop {
                if intro_done_event.consume() {
                    break;
                }

                let frame_start = SystemTime::now();

                gui_context.canvas().set_draw_color(Color::BLACK);
                gui_context.canvas().clear();

                systems::work_timers::run(&mut world, &mut game_time);
                systems::update_animated_textures::run(&mut world, &mut game_time);
                systems::draw_textures::run(gui_context.canvas(), &mut world, &mut resources);
                systems::draw_texts::run(gui_context.canvas(), &mut world, &mut resources, &texture_creator);

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
