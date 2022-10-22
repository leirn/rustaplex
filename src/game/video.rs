use std::cell::RefCell;
use std::rc::Rc;
use super::graphics;

pub struct Video{
    canvas: sdl2::render::Canvas<sdl2::video::Window>,}

impl Video {
    pub fn init(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Video {
        let _video_subsystem = sdl_context.borrow_mut().video().unwrap();
        let _window = _video_subsystem.window("Window", graphics::K_SCREEN_WIDTH as u32, graphics::K_SCREEN_HEIGHT as u32)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let _canvas = _window.into_canvas()
            .index(Video::find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        Video {
            //video_subsystem: _video_subsystem,
            //window: _window,
            canvas: _canvas,
        }
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