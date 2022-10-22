pub mod graphics;
mod level;
mod video;

use graphics::Graphics;
use video::Video;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Game {
    graphics: Graphics,
    video: Video,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
}

impl Game {
    pub fn new() -> Game {
        let sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        Game {
            graphics: Graphics::init(),
            video: Video::init(sdl_context.clone()),
            sdl_context: sdl_context,
        }
    }

    pub fn start(&self) {
        self.init_logging();
        self.init_system();
        self.init_audio();
        self.init_video();
        self.init_controller();
        self.init_game_state_data();

        self.load_all_ressources();

        // Start main loop
        self.run();
    }

    fn run(&self) {
        let mut continuer = true;
        while continuer {
        let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown {
                        keycode: Some(Keycode::Q), ..
                    } => { continuer = false},
                    _ => ()
                }
            }
    }}

    fn load_all_ressources(&self) {}

    fn init_audio(&self) {}
    fn init_controller(&self) {}

    /// Init SDL
    fn init_system(&self) {}

    fn init_video(&self) {}
    fn init_logging(&self) {}
    fn init_game_state_data(&self) {}
}
