use sdl2::event::Event;
// use sdl2::render::Canvas;
// use sdl2::video;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
// apt install libsdl2-image-dev
use std::path::Path;
use std::thread;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    //let _audio_subsystem = sdl_context.audio()?; // TODO use
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?; // does this just enable .png support like that?

    print_display_information(&video_subsystem);
    // TODO set display width/height from Desktop_display_mode()

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
    // let bee_idiot_spritesheet = texture_creator.load_texture(Path::new("res/beehive_dumbass.png"))?; // TODO implement bee idiot

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        display_intro(&mut canvas);

        display_rectangle_grid(&mut canvas);

        display_flying_huhns(&mut canvas);
    }

    Ok(())
}

pub fn print_display_information(video: &sdl2::VideoSubsystem) {
    println!("\n");
    println!(
        "default is_screen_saver_enabled: {}",
        video.is_screen_saver_enabled()
    );
    video.disable_screen_saver();
    println!(
        "disabled is_screen_saver_enabled: {}",
        video.is_screen_saver_enabled()
    );
    video.enable_screen_saver();
    println!(
        "enabled is_screen_saver_enabled: {}",
        video.is_screen_saver_enabled()
    );
    println!();

    println!("Information hardcoded for display_index 0!");
    println!(
        "current display 0 hz, w, h: {} {} {}",
        video.current_display_mode(0).unwrap().refresh_rate,
        video.current_display_mode(0).unwrap().w,
        video.current_display_mode(0).unwrap().h
    );
    println!("current_video_driver: {}", video.current_video_driver());
    println!(
        "desktop display 0 hz, w, h: {} {} {}",
        video.desktop_display_mode(0).unwrap().refresh_rate,
        video.desktop_display_mode(0).unwrap().w,
        video.desktop_display_mode(0).unwrap().h
    );
    println!(
        "display 0 hz: {}",
        video.display_mode(0, 4).unwrap().refresh_rate
    );
    println!("display 0 name: {}", video.display_name(0).unwrap());
}

pub fn draw_nth_rectangle(
    canvas: &mut sdl2::render::WindowCanvas,
    x: i32,
    y: i32,
    x_nth: u32,
    y_nth: u32,
) {
    let width = SCREEN_WIDTH / x_nth;
    let height = SCREEN_HEIGHT / y_nth;

    let mut x_border = 0;
    let mut y_border = 0;
    if width > 10 {
        x_border = 5;
    }
    if height > 6 {
        y_border = 3;
    }

    canvas
        .fill_rect(Rect::new(x, y, width - x_border, height - y_border))
        .unwrap();
}

pub fn display_intro(canvas: &mut sdl2::render::WindowCanvas) {
    const HIH_NUM_ANIMS: u32 = 14;
    const HIH_HEIGHT_ANIM: u32 = 1624 / 14;

    let texture_creator = canvas.texture_creator();
    let huhn_in_hole_spritesheet = texture_creator
        .load_texture(Path::new("../../res/intro_huhn_in_hole.png"))
        .unwrap(); // https://onlinetools.com/image/remove-specific-color-from-image

    let sprite_tile_size = (76, HIH_HEIGHT_ANIM);

    // Huhn in hole - sprite animation
    let mut source_rect_hih = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.1);
    let mut dest_rect_hih = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.1 * 4);
    dest_rect_hih.center_on(Point::new(
        (SCREEN_WIDTH as i32) / 2,
        (SCREEN_HEIGHT as i32) / 2,
    )); // wo fängt das Rect an aufm Bildschirm

    for huhn_in_hole_anim_counter in 0..HIH_NUM_ANIMS {
        // set the current frame for time
        source_rect_hih.set_y((huhn_in_hole_anim_counter * HIH_HEIGHT_ANIM) as i32); // ! move the source in the spritesheet
        // dest_rect_hih.set_y(huhn_in_hole_anim_counter); // ! move the box in the display

        canvas.clear();
        // copy the frame to the canvas
        canvas
            .copy_ex(
                &huhn_in_hole_spritesheet,
                Some(source_rect_hih),
                Some(dest_rect_hih),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
        canvas.present();

        if huhn_in_hole_anim_counter >= 8 {
            // wait extra for slower anim
            thread::sleep(Duration::from_millis((1000 / 60) * 5));
        }

        thread::sleep(Duration::from_millis((1000 / 60) * 15));
    }
}

pub fn display_rectangle_grid(canvas: &mut sdl2::render::WindowCanvas) {
    let columns = 10;
    let rows = 10;
    for row in 0..rows {
        for column in 0..columns {
            if column == 0 {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
            }
            canvas.set_draw_color(Color::WHITE);
            let x_frac = 1.0 / (columns as f64) * (column as f64);
            let y_frac = 1.0 / (rows as f64) * (row as f64);
            let x = (SCREEN_WIDTH as f64 * x_frac).round() as i32;
            let y = (SCREEN_HEIGHT as f64 * y_frac).round() as i32;
            draw_nth_rectangle(canvas, x, y, columns, rows);
            canvas.present();
            // thread::sleep(Duration::from_millis(5));
        }
    }
}

pub fn display_flying_huhns(canvas: &mut sdl2::render::WindowCanvas) {
    const FLYING_NUM_ANIMS: u32 = 13;
    const _DYING_NUM_ANIMS: u32 = 8;

    const CLOSE_HEIGHT_SPRITE: u32 = 1976 / FLYING_NUM_ANIMS;
    const CLOSE_WIDTH_SPRITE: u32 = 140;
    const MEDIUM_HEIGHT_SPRITE: u32 = 1560 / FLYING_NUM_ANIMS;
    const MEDIUM_WIDTH_SPRITE: u32 = 120;
    const FAR_HEIGHT_SPRITE: u32 = 416 / FLYING_NUM_ANIMS;
    const FAR_WIDTH_SPRITE: u32 = 44;

    let texture_creator = canvas.texture_creator();
    let huhns_fyling_dying_spritesheet = texture_creator
        .load_texture(Path::new("../../res/flying_huhn.png"))
        .unwrap(); // https://onlinetools.com/image/remove-specific-color-from-image

    let sprite_tile_size_close = (CLOSE_WIDTH_SPRITE, CLOSE_HEIGHT_SPRITE);
    let sprite_tile_size_medium = (MEDIUM_WIDTH_SPRITE, MEDIUM_HEIGHT_SPRITE);
    let sprite_tile_size_far = (FAR_WIDTH_SPRITE, FAR_HEIGHT_SPRITE);

    // sprite animation sources
    let mut temp_x = 2;
    let mut source_rect_close_flying = Rect::new(
        temp_x as i32,
        2,
        sprite_tile_size_close.0,
        sprite_tile_size_close.1,
    );
    temp_x += CLOSE_WIDTH_SPRITE + 2;
    //let mut source_rect_close_dying = Rect::new(temp_x as i32, 2, sprite_tile_size_close.0, sprite_tile_size_close.1);
    temp_x += CLOSE_WIDTH_SPRITE + 2;
    let mut source_rect_medium_flying = Rect::new(
        temp_x as i32,
        2,
        sprite_tile_size_medium.0,
        sprite_tile_size_medium.1,
    );
    temp_x += MEDIUM_WIDTH_SPRITE + 2;
    //let source_rect_medium_dying = Rect::new(temp_x as i32, 2, sprite_tile_size_medium.0, sprite_tile_size_medium.1);
    temp_x += MEDIUM_WIDTH_SPRITE + 2;
    let mut source_rect_far_flying = Rect::new(
        temp_x as i32,
        2,
        sprite_tile_size_far.0,
        sprite_tile_size_far.1,
    );
    //temp_x += FAR_WIDTH_SPRITE + 2;
    //let source_rect_far_dying = Rect::new(temp_x as i32, 2, sprite_tile_size_far.0, sprite_tile_size_far.1);

    // sprite animation destinations
    let mut dest_rect_close = Rect::new(
        0,
        0,
        sprite_tile_size_close.0 * 4,
        sprite_tile_size_close.1 * 4,
    );
    let mut dest_rect_medium = Rect::new(
        0,
        0,
        sprite_tile_size_medium.0 * 4,
        sprite_tile_size_medium.1 * 4,
    );
    let mut dest_rect_far = Rect::new(0, 0, sprite_tile_size_far.0 * 4, sprite_tile_size_far.1 * 4);

    // sprite animation destinations starting positions
    dest_rect_close.center_on(Point::new(
        CLOSE_WIDTH_SPRITE as i32 * -4,
        ((SCREEN_HEIGHT as f64) * 0.6).round() as i32,
    ));
    dest_rect_medium.center_on(Point::new(
        MEDIUM_WIDTH_SPRITE as i32 * -4,
        ((SCREEN_HEIGHT as f64) * 0.4).round() as i32,
    ));
    dest_rect_far.center_on(Point::new(
        FAR_WIDTH_SPRITE as i32 * -4,
        ((SCREEN_HEIGHT as f64) * 0.2).round() as i32,
    ));

    const NUM_OF_REPS: u32 = 10; // TODO make them die after the reps in another for loop or smth
    const TRAVEL_SPEED_DIVIDER: u32 = 10;

    for flying_anim_counter in 0..FLYING_NUM_ANIMS * NUM_OF_REPS {
        let anim_num = flying_anim_counter % FLYING_NUM_ANIMS;

        // move the source in the spritesheet
        source_rect_close_flying.set_y((anim_num * CLOSE_HEIGHT_SPRITE + 2) as i32);
        source_rect_medium_flying.set_y((anim_num * MEDIUM_HEIGHT_SPRITE + 2) as i32);
        source_rect_far_flying.set_y((anim_num * FAR_HEIGHT_SPRITE + 2) as i32);
        // move the box in the display
        dest_rect_close
            .set_x(((flying_anim_counter * CLOSE_WIDTH_SPRITE) / TRAVEL_SPEED_DIVIDER) as i32);
        dest_rect_medium
            .set_x(((flying_anim_counter * MEDIUM_WIDTH_SPRITE) / TRAVEL_SPEED_DIVIDER) as i32);
        dest_rect_far
            .set_x(((flying_anim_counter * FAR_WIDTH_SPRITE) / TRAVEL_SPEED_DIVIDER) as i32);

        canvas.clear();
        // copy the frame to the canvas
        canvas
            .copy_ex(
                &huhns_fyling_dying_spritesheet,
                Some(source_rect_close_flying),
                Some(dest_rect_close),
                0.0,
                None,
                true,
                false,
            )
            .unwrap();
        canvas
            .copy_ex(
                &huhns_fyling_dying_spritesheet,
                Some(source_rect_medium_flying),
                Some(dest_rect_medium),
                0.0,
                None,
                true,
                false,
            )
            .unwrap();
        canvas
            .copy_ex(
                &huhns_fyling_dying_spritesheet,
                Some(source_rect_far_flying),
                Some(dest_rect_far),
                0.0,
                None,
                true,
                false,
            )
            .unwrap();
        canvas.present();

        thread::sleep(Duration::from_millis((1000 / 60) * 4));
    }
}
