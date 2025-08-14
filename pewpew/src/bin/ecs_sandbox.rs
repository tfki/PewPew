use hecs::World;
use pewpew::gui::components::{texture, Hitbox, Location};
use pewpew::gui::resources::Resources;
use pewpew::gui::systems;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    //let _audio_subsystem = sdl_context.audio()?; // TODO use
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?; // does this just enable .png support like that?

    let window = video_subsystem
        .window("PewPew sdl-sandbox", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        // .software()
        .build()
        .map_err(|e| e.to_string())?;

    // let texture_creator = canvas.texture_creator(); // TODO implement bee idiot
    // let bee_idiot_spritesheet = texture_creator.load_texture(Path::new("ressources/Moorhuhn_X_Huhn_at_Beehive.png"))?; // TODO implement bee idiot

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let _event_pump = sdl_context.event_pump()?;

    display_intro(&mut canvas);

    Ok(())
}

pub fn display_intro(canvas: &mut sdl2::render::WindowCanvas) {
    let texture_creator = canvas.texture_creator();
    {
        let mut resources = Resources::default();

        resources.images.push(
            texture_creator
                .load_texture(Path::new("ressources/Huhn_in_Hole.png"))
                .unwrap(),
        ); // https://onlinetools.com/image/remove-specific-color-from-image

        let mut world = World::new();

        for x in 1..(SCREEN_WIDTH / 100) {
            for y in 1..(SCREEN_HEIGHT / 100) {
                let texture = texture::Builder::new(0)
                    .with_num_frames(14)
                    .looping()
                    .with_frame_advance_interval(Duration::from_millis((50 * (1 + x + y)) as u64))
                    .build();
                let location = Location {
                    x: x * 100,
                    y: y * 100,
                };

                if (x + y) % 2 == 0 {
                    world.spawn((texture, location));
                } else {
                    world.spawn((
                        texture,
                        location,
                        Hitbox {
                            width: 100,
                            height: 100,
                        },
                    ));
                }
            }
        }

        {
            for frame in 0..1000 {
                let frame_start = SystemTime::now();

                canvas.set_draw_color(Color::BLACK);
                canvas.clear();

                if frame % 100 == 0 {
                    // do nothing, black screen
                } else if (frame + 1) % 100 == 0 || (frame + 2) % 100 == 0 || (frame + 3) % 100 == 0 {
                    // then, for three frames, show hitboxes
                    systems::draw_hitboxes::run(canvas, &mut world);
                } else {
                    systems::draw_textures::run(canvas, &mut world, &mut resources);
                }

                canvas.present();

                let frame_end = SystemTime::now();
                let frame_duration = frame_end.duration_since(frame_start).unwrap();
                let wait_duration = Duration::from_millis(33_u128.saturating_sub(frame_duration.as_millis()) as u64);
                println!("frame took {frame_duration:?}, waiting {wait_duration:?} until next frame");

                thread::sleep(wait_duration);
            }
        }
    }
}
