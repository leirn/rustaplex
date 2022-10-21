mod graphics;
mod level;
use graphics::Graphics;
pub struct Game {
    graphics: Graphics,
}

impl Game {
    pub fn new() -> Game {
        Game {
            graphics: Graphics::init(),
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

    fn run(&self) {}

    fn load_all_ressources(&self) {}

    fn init_audio(&self) {}
    fn init_controller(&self) {}

    /// Init SDL
    fn init_system(&self) {}

    fn init_video(&self) {}
    fn init_logging(&self) {}
    fn init_game_state_data(&self) {}
}
