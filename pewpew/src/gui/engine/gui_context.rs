use sdl2::image::InitFlag;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::common::cancel_token::CancelToken;
use crate::comm::gui::GuiComm;

pub struct Settings {
    width: Option<i32>,
    height: Option<i32>,
    fullscreen: bool, // if fullscreen == true, width and height have no effect
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            width: None,
            height: None,
            fullscreen: true,
        }
    }
}

impl Settings {
    pub fn with_dimensions(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn windowed(mut self) -> Self {
        self.fullscreen = false;
        self
    }
}

pub struct GuiContext {
    // underscore because it is never accessed
    // but we need it here, so it gets dropped with the canvas
    _sdl_context: Sdl,
    canvas: WindowCanvas,
    cancel_token: CancelToken,
    comm: GuiComm,
}

impl GuiContext {
    pub fn new(settings: Settings, cancel_token: CancelToken, comm: GuiComm) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let _screen_refresh_rate = video.desktop_display_mode(0).unwrap().refresh_rate;
        let screen_width = video.desktop_display_mode(0).unwrap().w;
        let screen_height = video.desktop_display_mode(0).unwrap().h;

        let mut window_builder = video.window(
            "PewPew sdl-sandbox",
            settings.width.unwrap_or(screen_width) as u32,
            settings.height.unwrap_or(screen_height) as u32,
        );

        if settings.fullscreen {
            window_builder.fullscreen_desktop();
        }

        let window = window_builder
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync() //< this means the screen cannot
            // render faster than your display rate (usually 60Hz or 144Hz)
            // .software()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        GuiContext {
            _sdl_context: sdl_context,
            canvas,
            cancel_token,
            comm,
        }
    }

    pub fn canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }

    pub fn comm(&mut self) -> &mut GuiComm {
        &mut self.comm
    }

    pub fn cancel_token(&mut self) -> &mut CancelToken {
        &mut self.cancel_token
    }
}
