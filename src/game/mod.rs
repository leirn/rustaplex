pub mod graphics;
mod level;
mod video;
mod globals;
mod utils;

use globals::*;
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
    g_current_level_state_with_padding:[StatefulLevelTile; K_LEVEL_DATA_LENGTH + K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING], 
    g_frame_counter: u16,
    g_random_generator_seed: u16,
    
}

impl Game {
    pub fn new() -> Game {
        let sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        Game {
            graphics: Graphics::init(),
            video: Video::init(sdl_context.clone()),
            sdl_context: sdl_context,
            g_current_level_state_with_padding:[StatefulLevelTile::default(); K_LEVEL_DATA_LENGTH + K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING],
            g_frame_counter: 0, 
            g_random_generator_seed: 0,
        }
    }

    pub fn start(&mut self) {
        // Based from open-supaplex
        // parseCommandLineOptions(argc, argv); --> Not used yet
        //initializeLogging(); --> No logging system
        // initializeSystem(); --> Initialize SDL. Already done
        // initializeVideo(gFastMode); --> Initialise SDL video, ALready done
        // initializeControllers(); --> For SDL Joystick, not implemented yet
        // initializeAudio(); --> TODO
        // readAdvancedConfig(); --> TODO : what is the use ?

        // handleSystemEvent(); --> SDL Quit event handling. Will be manage later in the loop, no use so early in the game

        self.init_game_state_data();

        self.generate_random_seed_from_clock();

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


    fn init_video(&self) {}
    fn init_logging(&self) {}

    /// Initalise tile states
    fn init_game_state_data(&mut self) {
        // Initialize game state with the same values as in the original game
        const  K_LEVEL_STATE_PRECEDING_PADDING: [u16; K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING] = [
            0x8995 , 0x8995 , 0x8995 , 0x8a3b , 0x8a3b , 0x8a3b , 0x8a3b , 0x8a3b ,
            0x8a3b , 0x8a3b , 0x8a3b , 0x8ae8 , 0x8ae8 , 0x8ae8 , 0x8ae8 , 0x8ae8 ,
            0x8ae8 , 0x8ae8 , 0x8ae8 , 0x8bb1 , 0x8bb1 , 0x8bb1 , 0x8bb1 , 0x8bb1 ,
            0x8bb1 , 0x8bb1 , 0x8bb1 , 0x8c85 , 0x8c85 , 0x8c85 , 0x8c85 , 0x8c85 ,
            0x8c85 , 0x8c85 , 0x8c85 , 0x8d5b , 0x8d5b , 0x8d5b , 0x8d5b , 0x8d5b ,
            0x8d5b , 0x8d5b , 0x8d5b , 0x8e06 , 0x8e06 , 0x8e06 , 0x8e06 , 0x8e06 ,
            0x8e06 , 0x8e06 , 0x8e06 , 0x8eac , 0x8eac , 0x8eac , 0x8eac , 0x8eac ,
            0x8eac , 0x8eac , 0x8eac , 0x8f59 , 0x8f59 , 0x8f59 , 0x8f59 , 0x8f59 ,
            0x8f59 , 0x8f59 , 0x8f59 , 0x0000 , 0x1370 , 0x0000 , 0x0000 , 0x17e8 ,
            0x0000 , 0x0000 , 0x0000 , 0x3869 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x86d0 , 0x0000 , 0x34b2 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x8b8f , 0x341d , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x3923 , 0x0909 , 0x0c00 , 0x0800 , 0x5800 , 0x0000 ,
            0x0000 , 0x2500 , 0x0677 , 0x007f , 0x0000 , 0x0001 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0xec00 , 0x2606 , 0x0005 , 0x0000 ,
            0x0000 , 0x0100 , 0x0000 , 0x0000 , 0x3231 , 0x3433 , 0x3635 , 0x3837 ,
            0x3039 , 0x002d , 0x0008 , 0x5751 , 0x5245 , 0x5954 , 0x4955 , 0x504f ,
            0x0000 , 0x000a , 0x5341 , 0x4644 , 0x4847 , 0x4b4a , 0x004c , 0x0000 ,
            0x0000 , 0x585a , 0x5643 , 0x4e42 , 0x004d , 0x0000 , 0x0000 , 0x2000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x002e , 0x001e , 0x0031 , 0x0014 , 0x0039 ,
            0x001f , 0x0014 , 0x0018 , 0xffff , 0x0001 , 0x4c01 , 0x5645 , 0x4c45 ,
            0x2e53 , 0x4144 , 0x0054 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 ,
            0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000 , 0x0000
        ];

        for idx in 0..K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING 
        {
            let value = K_LEVEL_STATE_PRECEDING_PADDING[idx];
            let value = crate::game::utils::convert_16le(value);
            self.g_current_level_state_with_padding[idx].tile = (value & 0xff) as u8;
            self.g_current_level_state_with_padding[idx].state = (value >> 8) as u8;
        }

        self.g_frame_counter = 0xf000;
    }

    /// Updates the random seed using the clock
    fn generate_random_seed_from_clock(&mut self) {
        let time_in_milliseconds: u32 = self.sdl_context.borrow_mut().timer().unwrap().ticks();
        // In order to keep the same behavior and values, this code will convert
        // the time in milliseconds to the clock count, as described in
        // http://vitaly_filatov.tripod.com/ng/asm/asm_029.1.html
        // If 1 second is 18.2 clock counts, we need to divide the time
        // by 1000 to get the seconds, and then multiply by 18.2.
        //
        // Using 182 and 10000 in the rust implementation to avoid integer/float behaviour
        let clock_count: u32 = time_in_milliseconds * 182 / 10000;
        let low_value:u16 = (clock_count & 0xffff) as u16;
        let high_value = ((clock_count >> 16) & 0xfff) as u16;
        self.g_random_generator_seed = high_value ^ low_value;
    }
}
