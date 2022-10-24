use super::graphics::{self, K_SCREEN_HEIGHT, K_SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::FullscreenType;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
enum ScalingMode {
    ScalingModeAspectFit,
    ScalingModeAspectFill,
    ScalingModeIntegerFactor,
    ScalingModeFullscreen,
    ScalingModeAspectCorrect,
    ScalingModeCount,
}

pub const TEXTURE_ASPECT_RATIO: f64 = K_SCREEN_WIDTH as f64 / K_SCREEN_HEIGHT as f64;

pub struct Video {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    g_scaling_mode: ScalingMode,
}

impl Video {
    pub fn init(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Video {
        let _video_subsystem = sdl_context.borrow_mut().video().unwrap();
        let _window = _video_subsystem
            .window(
                "Window",
                graphics::K_SCREEN_WIDTH as u32,
                graphics::K_SCREEN_HEIGHT as u32,
            )
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .resizable()
            .build()
            .unwrap();
        let mut _canvas = _window
            .into_canvas()
            .index(Video::find_sdl_gl_driver().unwrap())
            .present_vsync()
            .build()
            .unwrap();

        _canvas.set_draw_color(Color::RGB(255, 210, 0));
        // A draw a rectangle which almost fills our window with it !
        _canvas
            .fill_rect(Rect::new(
                10,
                10,
                K_SCREEN_WIDTH as u32 - 20,
                K_SCREEN_HEIGHT as u32 - 20,
            ))
            .unwrap();
        _canvas.present();

        Video {
            //video_subsystem: _video_subsystem,
            canvas: _canvas,
            g_scaling_mode: ScalingMode::ScalingModeAspectFit,
        }
    }

    pub fn update_window_viewport(&mut self) {
        let (window_height, window_width) = self.canvas.window().size();

        // If the scaling mode is fullscreen, use the window size
        if self.g_scaling_mode == ScalingMode::ScalingModeFullscreen {
            self.canvas.viewport().set_x(0);
            self.canvas.viewport().set_y(0);
            self.canvas.viewport().set_width(window_width);
            self.canvas.viewport().set_height(window_height);
            return;
        }

        let texture_aspect_ratio;
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectCorrect {
            texture_aspect_ratio = 4 as f64 / 3 as f64;
        } else {
            texture_aspect_ratio = TEXTURE_ASPECT_RATIO;
        }

        let mut max_viewport_width = window_width;
        let mut max_viewport_height = window_height;

        // For "integer factor" scaling, pick the highest integer factor that fits into the window
        if self.g_scaling_mode == ScalingMode::ScalingModeIntegerFactor {
            max_viewport_width =
                ((window_width as f64 / K_SCREEN_WIDTH as f64) * K_SCREEN_WIDTH as f64) as u32;
            max_viewport_height =
                ((window_height as f64 / K_SCREEN_HEIGHT as f64) * K_SCREEN_HEIGHT as f64) as u32;
        }

        // If the resulting viewport is too small, do proportional scaling according to the window size
        if max_viewport_width == 0 {
            max_viewport_width = window_width;
        }
        if max_viewport_height == 0 {
            max_viewport_height = window_height;
        }

        let screen_aspect_ratio = window_width as f64 / window_height as f64;
        let mut should_preserve_width = texture_aspect_ratio > screen_aspect_ratio;

        // The only difference between aspect fill and fit is that fit will leave black bars
        // and fill will crop the image.
        // TODO : does not seem to work, always in fit mode without preserving ratio
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectFill {
            should_preserve_width = !should_preserve_width;
        }

        if should_preserve_width {
            self.canvas
                .viewport()
                .set_x((window_width as i32 - max_viewport_width as i32) >> 1);
            self.canvas.viewport().set_width(max_viewport_width);
            let viewport_width = self.canvas.viewport().width();
            self.canvas
                .viewport()
                .set_height(viewport_width / texture_aspect_ratio as u32);
            let viewport_height = self.canvas.viewport().height();
            self.canvas
                .viewport()
                .set_y((window_height as i32 - viewport_height as i32) >> 1);
        } else {
            self.canvas
                .viewport()
                .set_y((window_height as i32 - max_viewport_height as i32) >> 1);
            self.canvas.viewport().set_height(max_viewport_height);
            let viewport_height = self.canvas.viewport().height();
            self.canvas
                .viewport()
                .set_width(viewport_height * texture_aspect_ratio as u32);
            let viewport_width = self.canvas.viewport().width();
            self.canvas
                .viewport()
                .set_x((window_width as i32 - viewport_width as i32) >> 1);
        }

        self.canvas.present();
    }

    pub fn getScalingMode(&self) -> ScalingMode {
        self.g_scaling_mode
    }

    pub fn setScalingMode(&mut self, mode: ScalingMode) {
        self.g_scaling_mode = mode;
        self.update_window_viewport();
    }

    pub fn toggle_fullscreen(&mut self) {
        self.set_fullscreen_mode(!self.get_fullscreen_mode());
    }

    fn set_fullscreen_mode(&mut self, fullscreen: bool) {
        let full_screen_type = match fullscreen {
            true => FullscreenType::True,
            false => FullscreenType::Desktop,
        };
        self.canvas
            .window_mut()
            .set_fullscreen(full_screen_type)
            .unwrap();
    }

    fn get_fullscreen_mode(&self) -> bool {
        let is_fullscreen = self.canvas.window().fullscreen_state();
        is_fullscreen == FullscreenType::True
    }

    /// Find SDL GL Driver to initiate SDL window
    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }
}
