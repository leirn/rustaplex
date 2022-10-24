mod globals;
pub mod graphics;
mod level;
pub mod video;
mod utils;

use globals::*;
use graphics::{G_TITLE_PALETTE_DATA, Graphics};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::rc::Rc;
use video::Video;

pub struct Game<'a> {
    graphics: Graphics<'a>,
    video: Rc<RefCell<Video<'a>>>,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    g_current_level_state_with_padding:
        [StatefulLevelTile; K_LEVEL_DATA_LENGTH + K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING],
    g_frame_counter: u16,
    g_random_generator_seed: u16,
    g_level_list_data: [String; K_NUMBER_OF_LEVEL_WITH_PADDING],
    g_player_list_data: [PlayerEntry; K_NUMBER_OF_PLAYERS],
    g_hall_of_fame_data: [HallOfFameEntry; K_NUMBER_OF_HALL_OF_FAME_ENTRIES],
}

impl Game<'_> {
    pub fn new() -> Game<'static> {
        let sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        let video= Rc::new(RefCell::new(Video::init(sdl_context.clone())));
        Game {
            video: video.clone(),
            graphics: Graphics::init(video.clone(), sdl_context.clone()),
            sdl_context: sdl_context,
            g_current_level_state_with_padding: [StatefulLevelTile::default();
                K_LEVEL_DATA_LENGTH + K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING],
            g_frame_counter: 0,
            g_random_generator_seed: 0,
            g_level_list_data: [(); K_NUMBER_OF_LEVEL_WITH_PADDING]
                .map(|_| String::from("                           ")),
            g_player_list_data: [(); K_NUMBER_OF_PLAYERS].map(|_| PlayerEntry::new()),
            g_hall_of_fame_data: [(); K_NUMBER_OF_HALL_OF_FAME_ENTRIES]
                .map(|_| HallOfFameEntry::new()),
        }
    }

    pub fn start(&mut self) {
        // Based from open-supaplex
        // parseCommandLineOptions(argc, argv); --> Not used yet
        // initializeLogging(); --> No logging system
        // initializeSystem(); --> Initialize SDL. Already done
        // initializeVideo(gFastMode); --> Initialise SDL video, ALready done
        // initializeControllers(); --> For SDL Joystick, not implemented yet
        // initializeAudio(); --> TODO
        // readAdvancedConfig(); --> TODO : what is the use ?

        // handleSystemEvent(); --> SDL Quit event handling. Will be manage later in the loop, no use so early in the game

        self.init_game_state_data();

        self.generate_random_seed_from_clock();

        self.initialize_fade_palette();
        self.graphics.video_loop();
        self.graphics.read_and_render_title_dat();


        let title_dat_palette = Graphics::convert_palette_data_to_palette(G_TITLE_PALETTE_DATA);
        self.graphics.fade_to_palette(title_dat_palette);

        self.load_all_ressources();// Equivalent to Read everything


        // Opening sequence
        //self.load_screen_2();
        // Start main loop
        self.run();
    }

    fn run(&mut self) {
        let mut continuer = true;
        while continuer {
            let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => continuer = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::F),
                        ..
                    } => self.video.borrow_mut().toggle_fullscreen(),
                    Event::Window { win_event, .. } => {
                        if let WindowEvent::Resized(_w, _h) = win_event {
                            //handle the resize event
                            println!("Window resized Event received");
                            self.video.borrow_mut().update_window_viewport();
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn load_all_ressources(&mut self) {
        self.read_levels_lst();
        self.read_demo_files();
        self.read_hall_fame_lst();
        self.read_players_lst();
    }

    fn load_screen_2(&mut self) {
        self.graphics.read_and_render_title1_dat();
        // TODO unfinished
    }

    fn read_levels_lst(&mut self) {
        // Re-init g_level_list_data
        self.g_level_list_data = [(); K_NUMBER_OF_LEVEL_WITH_PADDING]
            .map(|_| String::from("                           "));
        self.g_level_list_data[K_LAST_LEVEL_INDEX] = String::from("- REPLAY SKIPPED LEVELS!! -");
        self.g_level_list_data[K_LAST_LEVEL_INDEX + 1] =
            String::from("---- UNBELIEVEABLE!!!! ----");

        let path = format!("{}/{}", RESSOURCES_PATH, G_LEVELS_DAT_FILENAME);
        let level_lst_file_path = Path::new(&path);
        match level_lst_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_LEVELS_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", level_lst_file_path.canonicalize()),
        }
        let mut file = File::open(level_lst_file_path)
            .expect(format!("Error while opening {}", G_LEVELS_DAT_FILENAME).as_str());

        let mut file_data = [0_u8; K_LEVEL_NAME_LENGTH - 1];

        for i in 0..K_NUMBER_OF_LEVEL {
            let seek_offset = 0x5A6 + i * K_LEVEL_DATA_LENGTH;
            file.seek(SeekFrom::Start(seek_offset as u64)).expect(
                format!(
                    "Error while seeking offset {} in {}",
                    seek_offset, G_LEVELS_LST_FILENAME
                )
                .as_str(),
            );
            file.read(&mut file_data)
                .expect(format!("Error while reading {}", G_LEVELS_LST_FILENAME).as_str());

            let level_name = format!("{:03} {:?}", i, file_data);

            self.g_level_list_data[i] = level_name;
        }
    }

    fn read_demo_files(&mut self) {}

    /// Read the list of players in hall of fame file
    fn read_hall_fame_lst(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_HALL_OF_FAME_LST_FILENAME);
        let hof_lst_file_path = Path::new(&path);
        match hof_lst_file_path.try_exists().expect(
            format!(
                "Can't check existence of file {}",
                G_HALL_OF_FAME_LST_FILENAME
            )
            .as_str(),
        ) {
            true => (),
            false => return, // No player file found
        }
        let mut file = File::open(hof_lst_file_path)
            .expect(format!("Error while opening {}", G_HALL_OF_FAME_LST_FILENAME).as_str());

        let mut player_data: [u8; K_HALL_OF_FAME_ENTRY_SIZE] = [0; K_HALL_OF_FAME_ENTRY_SIZE];
        for i in 0..K_NUMBER_OF_HALL_OF_FAME_ENTRIES {
            match file.read_exact(&mut player_data) {
                Ok(_) => (),
                Err(_) => return, // No more players to load
            }

            self.g_hall_of_fame_data[i].name = format!(
                "{}{}{}{}{}{}{}{}",
                player_data[0],
                player_data[1],
                player_data[2],
                player_data[3],
                player_data[4],
                player_data[5],
                player_data[6],
                player_data[7]
            );
            self.g_hall_of_fame_data[i].hours = player_data[K_PLAYER_NAME_LENGTH + 2];
            self.g_hall_of_fame_data[i].minutes = player_data[K_PLAYER_NAME_LENGTH + 3];
            self.g_hall_of_fame_data[i].seconds = player_data[K_PLAYER_NAME_LENGTH + 4];
        }
    }

    /// Read the PLAYER.DAT file to load previous player save.
    fn read_players_lst(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_PLAYERS_LST_FILENAME);
        let player_lst_file_path = Path::new(&path);
        match player_lst_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_PLAYERS_LST_FILENAME).as_str())
        {
            true => (),
            false => return, // No player file found
        }
        let mut file = File::open(player_lst_file_path)
            .expect(format!("Error while opening {}", G_PLAYERS_LST_FILENAME).as_str());

        let mut player_data: [u8; K_PLAYER_ENTRY_SIZE] = [0; K_PLAYER_ENTRY_SIZE];
        for i in 0..K_NUMBER_OF_PLAYERS {
            match file.read_exact(&mut player_data) {
                Ok(_) => (),
                Err(_) => return, // No more players to load
            }

            self.g_player_list_data[i].name = format!(
                "{}{}{}{}{}{}{}{}",
                player_data[0],
                player_data[1],
                player_data[2],
                player_data[3],
                player_data[4],
                player_data[5],
                player_data[6],
                player_data[7]
            );
            self.g_player_list_data[i].hours = player_data[K_PLAYER_NAME_LENGTH + 2];
            self.g_player_list_data[i].minutes = player_data[K_PLAYER_NAME_LENGTH + 3];
            self.g_player_list_data[i].seconds = player_data[K_PLAYER_NAME_LENGTH + 4];
            for j in 0..K_NUMBER_OF_LEVEL {
                self.g_player_list_data[i].level_state[j] =
                    player_data[K_PLAYER_NAME_LENGTH + 5 + j];
            }
            self.g_player_list_data[i].unknown1 =
                player_data[K_PLAYER_NAME_LENGTH + K_NUMBER_OF_LEVEL + 5];
            self.g_player_list_data[i].unknown2 =
                player_data[K_PLAYER_NAME_LENGTH + K_NUMBER_OF_LEVEL + 6];
            self.g_player_list_data[i].unknown3 =
                player_data[K_PLAYER_NAME_LENGTH + K_NUMBER_OF_LEVEL + 7];
            self.g_player_list_data[i].next_level_to_play =
                player_data[K_PLAYER_NAME_LENGTH + K_NUMBER_OF_LEVEL + 8];
            self.g_player_list_data[i].completed_all_levels =
                player_data[K_PLAYER_NAME_LENGTH + K_NUMBER_OF_LEVEL + 9];
        }
    }

    fn init_audio(&self) {}
    fn init_controller(&self) {}

    fn init_video(&self) {}
    fn init_logging(&self) {}

    /// Initalise tile states
    fn init_game_state_data(&mut self) {
        // Initialize game state with the same values as in the original game
        const K_LEVEL_STATE_PRECEDING_PADDING: [u16; K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING] = [
            0x8995, 0x8995, 0x8995, 0x8a3b, 0x8a3b, 0x8a3b, 0x8a3b, 0x8a3b, 0x8a3b, 0x8a3b, 0x8a3b,
            0x8ae8, 0x8ae8, 0x8ae8, 0x8ae8, 0x8ae8, 0x8ae8, 0x8ae8, 0x8ae8, 0x8bb1, 0x8bb1, 0x8bb1,
            0x8bb1, 0x8bb1, 0x8bb1, 0x8bb1, 0x8bb1, 0x8c85, 0x8c85, 0x8c85, 0x8c85, 0x8c85, 0x8c85,
            0x8c85, 0x8c85, 0x8d5b, 0x8d5b, 0x8d5b, 0x8d5b, 0x8d5b, 0x8d5b, 0x8d5b, 0x8d5b, 0x8e06,
            0x8e06, 0x8e06, 0x8e06, 0x8e06, 0x8e06, 0x8e06, 0x8e06, 0x8eac, 0x8eac, 0x8eac, 0x8eac,
            0x8eac, 0x8eac, 0x8eac, 0x8eac, 0x8f59, 0x8f59, 0x8f59, 0x8f59, 0x8f59, 0x8f59, 0x8f59,
            0x8f59, 0x0000, 0x1370, 0x0000, 0x0000, 0x17e8, 0x0000, 0x0000, 0x0000, 0x3869, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x86d0, 0x0000, 0x34b2, 0x0000,
            0x0000, 0x0000, 0x0000, 0x8b8f, 0x341d, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x3923,
            0x0909, 0x0c00, 0x0800, 0x5800, 0x0000, 0x0000, 0x2500, 0x0677, 0x007f, 0x0000, 0x0001,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0xec00, 0x2606, 0x0005, 0x0000, 0x0000, 0x0100, 0x0000,
            0x0000, 0x3231, 0x3433, 0x3635, 0x3837, 0x3039, 0x002d, 0x0008, 0x5751, 0x5245, 0x5954,
            0x4955, 0x504f, 0x0000, 0x000a, 0x5341, 0x4644, 0x4847, 0x4b4a, 0x004c, 0x0000, 0x0000,
            0x585a, 0x5643, 0x4e42, 0x004d, 0x0000, 0x0000, 0x2000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x002e, 0x001e, 0x0031, 0x0014, 0x0039, 0x001f, 0x0014, 0x0018,
            0xffff, 0x0001, 0x4c01, 0x5645, 0x4c45, 0x2e53, 0x4144, 0x0054, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000,
        ];

        for idx in 0..K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING {
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
        let low_value: u16 = (clock_count & 0xffff) as u16;
        let high_value = ((clock_count >> 16) & 0xfff) as u16;
        self.g_random_generator_seed = high_value ^ low_value;
    }

    fn initialize_fade_palette(&mut self) {
        self.graphics.set_palette(graphics::G_BLACK_PALETTE);
    }
}
