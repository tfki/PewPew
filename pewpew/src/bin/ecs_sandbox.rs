use hecs::World;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct AnimatedTexture {
    image_id: usize,
    num_frames: u32,
    current_frame: u32,
    repeat: bool,
    scale: f32,
    rotation_deg: f64,
}

struct Location {
    x: u32,
    y: u32,
}

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
    let mut world = World::new();

    // world.spawn((
    //     AnimatedTexture {
    //         image_id: 0,
    //         num_frames: 14,
    //         current_frame: 0,
    //         repeat: true,
    //         scale: 4.0,
    //         rotation_deg: 90.0,
    //     },
    //     Location { x: 100, y: 100 },
    // ));
    world.spawn((
        AnimatedTexture {
            image_id: 0,
            num_frames: 14,
            current_frame: 0,
            repeat: false,
            scale: 1.0,
            rotation_deg: 0.0,
        },
        Location { x: 100, y: 100 },
    ));

    let texture_creator = canvas.texture_creator();
    {
        let mut all_images = Vec::new();

        all_images.push(
            texture_creator
                .load_texture(Path::new("ressources/Huhn_in_Hole.png"))
                .unwrap(),
        ); // https://onlinetools.com/image/remove-specific-color-from-image

        for _ in 0..20 {
            canvas.clear();
            let mut to_be_despawned = Vec::new();

            for (id, (anim_texture, location)) in
                world.query_mut::<(&mut AnimatedTexture, &Location)>()
            {
                let sprite = &all_images[anim_texture.image_id];
                let tile_size = (
                    sprite.query().width,
                    sprite.query().height / anim_texture.num_frames,
                );

                // set the current frame by 'scrolling' vertically
                let source_rect = Rect::new(0, (anim_texture.current_frame * tile_size.1) as i32, tile_size.0, tile_size.1);

                let dest_rect = Rect::from_center(
                    Point::new(location.x as i32, location.y as i32),
                    (tile_size.0 as f32 * anim_texture.scale) as u32,
                    (tile_size.1 as f32 * anim_texture.scale) as u32,
                );

                canvas
                    .copy_ex(
                        &all_images[anim_texture.image_id],
                        Some(source_rect),
                        Some(dest_rect),
                        anim_texture.rotation_deg,
                        None,
                        false,
                        false,
                    )
                    .unwrap();

                if anim_texture.current_frame < anim_texture.num_frames {
                    anim_texture.current_frame += 1;
                } else if anim_texture.repeat {
                    anim_texture.current_frame = 0;
                } else {
                    to_be_despawned.push(id);
                }
            }

            for id in to_be_despawned {
                world.despawn(id).unwrap();
            }

            canvas.present();
            thread::sleep(Duration::from_millis(150));
        }
    }
}
