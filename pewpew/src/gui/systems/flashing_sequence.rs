use crate::comm::message::{GuiToHitreg, HitregToGui};
use crate::gui::components::Hitbox;
use crate::gui::gui_context::GuiContext;
use hecs::World;
use log::debug;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::{Duration, SystemTime};

fn usize_to_vec_bool(value: usize, max_idx: u32) -> Vec<bool> {
    let mut result = Vec::new();

    for i in 0..max_idx {
        let mask = 1 << i;
        result.push((mask & value) != 0);
    }

    result
}

pub fn run(gui_context: &mut GuiContext, world: &mut World, show_frames: bool) {
    debug!(target: "Gui Thread", "starting flashing sequence");

    let time_per_frame = Duration::from_millis(200);
    let all_hitboxes = world.query_mut::<&Hitbox>().into_iter().collect::<Vec<_>>();

    // ilog2 rounds down, but we need to round up, thus +1
    // and, pattern 0..0 is forbidden, thus another +1 = +2
    let num_frames = (all_hitboxes.len() + 2).ilog2();

    let sequences = all_hitboxes
        .iter()
        .enumerate()
        .map(|(i, (entity, _hitbox))| (*entity, usize_to_vec_bool(i + 1, num_frames)))
        .collect::<Vec<_>>();

    gui_context.canvas().set_draw_color(Color::BLACK);
    gui_context.canvas().clear();
    gui_context.canvas().present();

    gui_context
        .comm()
        .send(GuiToHitreg::FlashingSequenceStart {
            num_frames,
            sequences,
        })
        .unwrap();

    for frame in 0..num_frames {
        let frame_start = SystemTime::now();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();

        let frame_bitmask = 1_usize << frame;

        for (index, (_entity_id, hitbox)) in all_hitboxes.iter().enumerate() {
            if (index + 1) & frame_bitmask != 0 {
                gui_context.canvas().set_draw_color(Color::WHITE);
                gui_context
                    .canvas()
                    .fill_rect(hitbox.position.align_rect(hitbox.width, hitbox.height))
                    .unwrap();
            }
            if show_frames {
                gui_context.canvas().set_draw_color(Color::CYAN);
                gui_context
                    .canvas()
                    .draw_rect(hitbox.position.align_rect(hitbox.width, hitbox.height))
                    .unwrap();
            }
        }

        gui_context.canvas().present();
        let now = SystemTime::now();

        gui_context.comm().send(GuiToHitreg::Frame(now)).unwrap();

        let last_frame_duration = now.duration_since(frame_start).unwrap();
        let wait_duration = time_per_frame.saturating_sub(last_frame_duration);

        thread::sleep(wait_duration);
    }

    gui_context.canvas().set_draw_color(Color::BLACK);
    gui_context.canvas().clear();
    gui_context.canvas().present();

    // wait for answer from hitreg
    if let HitregToGui::Result(Some(victim)) = gui_context.comm().recv_from_hitreg().unwrap() {
        let hitbox = world.query_one_mut::<&Hitbox>(victim).unwrap();

        gui_context.canvas().set_draw_color(Color::BLACK);
        gui_context.canvas().clear();
        gui_context.canvas().set_draw_color(Color::RED);
        gui_context
            .canvas()
            .fill_rect(hitbox.position.align_rect(hitbox.width, hitbox.height))
            .unwrap();
        gui_context.canvas().present();
        thread::sleep(Duration::from_secs(1));
    }
}
