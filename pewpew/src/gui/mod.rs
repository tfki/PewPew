pub mod components;
mod gui_context;
pub mod resources;
pub mod systems;

use crate::common::cancel_token::CancelToken;
use crate::comm::GuiComm;
use crate::gui::components::movement::By;
use crate::gui::components::{movement, texture, Hitbox, Point};
use crate::gui::gui_context::GuiContext;
use crate::gui::resources::Resources;
use hecs::World;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn run(comm: GuiComm, cancel_token: CancelToken) {
    let mut gui_context = GuiContext::new(gui_context::Settings::default(), cancel_token, comm);

    display_intro(&mut gui_context);
}

fn display_intro(gui_context: &mut GuiContext) {
    let texture_creator = gui_context.canvas().texture_creator();
    {
        let mut resources = Resources::default();

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

        for x in 0..5 {
            for y in 0..5 {
                let anchor = Point {
                    x: x * 200,
                    y: y * 200,
                };
                let texture = texture::Builder::new(1, anchor)
                    .with_num_frames(13)
                    .with_vertical_flip()
                    .looping()
                    .with_frame_advance_interval(Duration::from_millis((10 * (10 - (x + y))) as u64))
                    .build();

                let movement = movement::Builder::new(
                    By {
                        x: x + y,
                        y: 0,
                    },
                    Duration::from_millis(33),
                )
                .build();

                world.spawn((
                    texture,
                    Hitbox {
                        anchor,
                        width: 200,
                        height: 200,
                    },
                    movement,
                ));
            }
        }

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
                    systems::flashing_sequence::run(gui_context, &mut world, true);
                } else {
                    systems::update_movements::run(&mut world);
                    systems::update_animated_textures::run(&mut world);
                    systems::draw_textures::run(gui_context.canvas(), &mut world, &mut resources);
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
