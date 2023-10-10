/*
* This file is part of the Rustaplex application (https://github.com/leirn/rustaplex).
* Copyright (c) 2022 Laurent Vromman <leirn@vromman.org>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful, but
* WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
* General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

pub mod animation;
mod button_borders;
mod demo;
mod game_states;
pub mod globals;
pub mod graphics;
mod keyboard;
pub mod level;
mod mouse;
mod sounds;
mod utils;
pub mod video;

use self::graphics::{G_BLACK_PALETTE, K_SCREEN_HEIGHT, K_SCREEN_WIDTH};
use button_borders::{
    ButtonStatus, K_MAIN_MENU_BUTTON_BORDERS, K_MAIN_MENU_BUTTON_DESCRIPTORS,
    K_NUMBER_OF_MAIN_MENU_BUTTONS,
};
use demo::DemoManager;
use game_states::GameStates;
use globals::*;
use graphics::{
    Graphics, PaletteType, G_TITLE1_PALETTE_DATA, G_TITLE2_PALETTE_DATA, G_TITLE_PALETTE_DATA,
    K_FULL_SCREEN_FRAMEBUFFER_LENGTH,
};
use keyboard::{Keys, UserInput, K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET};
use mouse::{Mouse, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_RIGHT};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use sounds::Sounds;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use video::Video;

pub struct Game<'a> {
    sounds: Sounds<'a>,
    graphics: Graphics<'a>,
    video: Rc<RefCell<Video<'a>>>,
    events: EventPump,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    g_random_generator_seed: u16,
    g_level_list_data: [String; K_NUMBER_OF_LEVEL_WITH_PADDING],
    g_player_list_data: [PlayerEntry; K_NUMBER_OF_PLAYERS],
    g_hall_of_fame_data: [HallOfFameEntry; K_NUMBER_OF_HALL_OF_FAME_ENTRIES],
    g_is_game_busy: bool,
    is_joystick_enabled: bool,
    demo_manager: DemoManager,
    states: GameStates,
    g_has_user_cheated: bool,
    g_should_autoselect_next_level_to_play: bool,
    g_is_forced_level: u8,
    g_is_playing_demo: bool,
    g_sp_demo_file_name: String,
    g_should_start_from_saved_snapshot: bool,
    word_58467: bool,
    byte_5A19B: bool,
    g_is_main_menu: bool,
    g_has_user_interrupted_demo: bool,
    g_selected_original_demo_level_number: u16,
    g_automatic_demo_playback_countdown: u16,
    g_should_exit_game: bool,
    g_ranking_text_entries: [String; K_NUMBER_OF_PLAYERS + 4],
    byte_58D47: u8,
    byte_58D46: u8,
    byte_59B83: bool,
    g_level_failed: bool,
    button_states: ButtonStatus,
    keyboard: Keys,
    mouse: Mouse,
    g_level_list_throttle_current_counter: u8,
    g_level_list_throttle_next_counter: u8,
    g_player_list_throttle_current_counter: u8,
    g_player_list_throttle_next_counter: u8,
    g_ranking_list_throttle_current_counter: u8,
    g_ranking_list_throttle_next_counter: u8,
    g_level_set_rotation_throttle_current_counter: u8,
    g_level_set_rotation_throttle_next_counter: u8,
}

impl Game<'_> {
    pub fn new() -> Game<'static> {
        let sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        let video = Rc::new(RefCell::new(Video::init(sdl_context.clone())));

        let events = sdl_context.borrow_mut().event_pump().unwrap();

        Game {
            sounds: Sounds::new(sdl_context.clone()),
            video: video.clone(),
            graphics: Graphics::init(video.clone(), sdl_context.clone()),
            events: events,
            sdl_context: sdl_context,
            g_random_generator_seed: 0,
            g_level_list_data: [(); K_NUMBER_OF_LEVEL_WITH_PADDING]
                .map(|_| String::from("                           ")),
            g_player_list_data: [(); K_NUMBER_OF_PLAYERS].map(|_| PlayerEntry::new()),
            g_hall_of_fame_data: [(); K_NUMBER_OF_HALL_OF_FAME_ENTRIES]
                .map(|_| HallOfFameEntry::new()),
            g_is_game_busy: false,
            is_joystick_enabled: false,
            demo_manager: DemoManager::new(),
            states: GameStates::new(),
            g_has_user_cheated: false,
            g_should_autoselect_next_level_to_play: false,
            g_is_forced_level: 0,
            g_is_playing_demo: false,
            g_sp_demo_file_name: String::new(),
            g_should_start_from_saved_snapshot: false,
            word_58467: true,
            byte_5A19B: false,
            g_is_main_menu: false,
            g_has_user_interrupted_demo: false,
            g_selected_original_demo_level_number: 0,
            g_automatic_demo_playback_countdown: 0,
            g_should_exit_game: false,
            g_ranking_text_entries: [(); K_NUMBER_OF_PLAYERS + 4].map(|_| String::new()),
            byte_58D47: 0,
            byte_58D46: 0,
            byte_59B83: false,
            g_level_failed: false,
            button_states: ButtonStatus::default(),
            keyboard: Keys::default(),
            g_level_list_throttle_current_counter: 0,
            g_level_list_throttle_next_counter: 0,
            g_player_list_throttle_current_counter: 0,
            g_player_list_throttle_next_counter: 0,
            g_ranking_list_throttle_current_counter: 0,
            g_ranking_list_throttle_next_counter: 0,
            g_level_set_rotation_throttle_current_counter: 0,
            g_level_set_rotation_throttle_next_counter: 0,
            mouse: Mouse::default(),
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

        self.splash_and_opening();
        // Start main loop
        self.run();
    }

    fn splash_and_opening(&mut self) {
        self.initialize_fade_palette();
        {
            // Display welcome grahpic
            self.graphics.video_loop();
            self.graphics.read_and_render_title_dat();
            let title_dat_palette = Graphics::convert_palette_data_to_palette(G_TITLE_PALETTE_DATA);
            self.graphics.fade_to_palette(title_dat_palette);

            // sleep a little to enjoy it
            for _ in 0..200 {
                self.handle_system_events();
                sleep(Duration::from_millis(10));
            }
        }

        self.load_all_ressources(); // Equivalent to Read everything

        {
            // Opening sequence
            self.load_screen_2();
            //readEverything(); // already done when loaded graphics component
            self.draw_speed_fix_title_and_version();
            self.graphics.open_credits_block();
            //openCreditsBlock(); // credits inside the block // 01ED:02C2
            //drawSpeedFixCredits();   // credits below the block (herman perk and elmer productions) // 01ED:02C5
            self.draw_speed_fix_credits();
        }
        self.read_config();

        // Wait for a key press to proceed
        self.wait_for_key_press_or_mouse_click();

        // Back in black
        self.graphics.fade_to_palette(G_BLACK_PALETTE);
    }

    fn run(&mut self) {
        let mut should_quit_the_game = false;
        let mut level_number_forced_to_load = 0_u8;
        while !should_quit_the_game {
            self.demo_manager.prepare_demo_recording_filename();
            let mut level_number_forced_to_load = 0_u8;

            if self.demo_manager.g_is_sp_demo_available_to_run == 2 {
                self.demo_manager.g_is_sp_demo_available_to_run = 1;
                if self.demo_manager.file_is_demo {
                    self.play_demo(0);
                } else {
                    self.demo_manager.g_is_playing_demo = false;
                }
                self.demo_manager.g_selected_original_demo_level_number = 0;
                self.g_has_user_cheated = true;
                self.demo_manager
                    .g_sp_demo_filename
                    .replace_range(3..6, "---");

                self.start_directly_from_level(1);
                continue;
            } else {
                level_number_forced_to_load = self.g_is_forced_level;
                self.g_is_forced_level = 0;
                self.g_is_playing_demo = false;
                if level_number_forced_to_load > 0 {
                    self.convert_level_number_to_3_digit_string_with_padding_0(
                        level_number_forced_to_load,
                    );
                }
            }

            if level_number_forced_to_load > 0 {
                self.start_directly_from_level(level_number_forced_to_load);
                continue;
            }
            if self.g_should_start_from_saved_snapshot {
                self.start_directly_from_level(1);
                continue;
            }

            self.g_has_user_cheated = false;
            self.run_main_menu();

            if should_quit_the_game {
                break;
            }
            /* TODO later since only in second cycle
                        self.read_levels();
                        self.graphics.fade_to_palette(G_BLACK_PALETTE);
                        self.g_is_game_busy = false;
                        self.draw_player_list();
                        self.initialize_game_info();
                        self.draw_fixed_level();
                        self.draw_game_panel(); // 01ED:0311
                        let number_of_infotrons: u16 = self.convert_to_easy_tiles();
                        self.reset_number_of_infotrons(number_of_infotrons);
                        self.find_murphy();
                        gCurrentPanelHeight = kPanelBitmapHeight;
                        drawCurrentLevelViewport(gCurrentPanelHeight); // Added by me
                        fadeToPalette(gGamePalette); // At this point the screen fades in and shows the game
            */
            if self.sounds.is_music_enabled == false {
                self.sounds.stop_music();
            }

            self.g_is_game_busy = true;
            //self.run_level();
            /*


            gIsSPDemoAvailableToRun = 0;
            if (gShouldExitGame != 0)
            {
                break; // goto loc_47067;
            }

            if (gFastMode != FastModeTypeNone)
            {
                break;
            }

            slideDownGameDash(); // 01ED:0351
            if (byte_59B71 != 0)
            {
                loadMurphySprites();
            }

            gIsGameBusy = 0;
            if (gShouldExitGame != 0)
            {
                break; // goto loc_47067;
            }
             */
            if self.sounds.is_music_enabled {
                self.sounds.play_music_if_needed();
            }

            self.graphics.video_loop();

            let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => should_quit_the_game = true,
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

    fn wait_for_key_press_or_mouse_click(&mut self) {
        loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::KeyUp { .. } | Event::MouseButtonUp { .. } => return,
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => std::process::exit(0),
                    _ => (),
                }

                sleep(Duration::from_millis(10))
            }
        }
    }

    fn handle_system_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => std::process::exit(0),
                _ => (),
            }
        }
    }

    fn play_demo(&mut self, demo_index: u16) {
        self.demo_manager.read_demo_files();

        self.g_random_generator_seed = self.demo_manager.g_demo_random_seeds[demo_index as usize];
        self.states.g_should_leave_main_menu = true;
        self.states.g_is_playing_demo = true;

        let mut demo_first_index: u16 =
            self.demo_manager.g_demos.demo_first_indices[demo_index as usize];

        if demo_first_index == 0xffff {
            self.states.g_should_leave_main_menu = false;
            self.states.g_is_playing_demo = false;
        }

        self.demo_manager.g_selected_original_demo_level_number = 0;

        let demo_level_number =
            self.demo_manager.g_demos.demo_data[demo_first_index as usize] as u16;
        let mut final_level_number = demo_index;

        if demo_level_number <= K_NUMBER_OF_LEVEL as u16 && demo_level_number != 0 {
            final_level_number = demo_level_number;
            self.demo_manager.g_selected_original_demo_level_number =
                (self.demo_manager.g_selected_original_demo_level_number & 0xFF00)
                    | final_level_number as usize;
        }
        self.demo_manager.g_demo_index_or_demo_level_number = final_level_number;

        demo_first_index += 1; // To skip the level number
        self.demo_manager.g_demo_current_input_index = demo_first_index;
        //    word_5A33C = demo_first_index;
        self.demo_manager.g_demo_current_input = UserInput::UserInputNone;
        self.demo_manager.g_demo_current_input_repeater_count = 1;
    }

    fn load_all_ressources(&mut self) {
        self.read_levels_lst();
        self.read_demo_files();
        self.read_hall_fame_lst();
        self.read_players_lst();
    }

    fn load_screen_2(&mut self) {
        self.graphics.read_and_render_title1_dat();
        let title1_dat_palette = Graphics::convert_palette_data_to_palette(G_TITLE1_PALETTE_DATA);
        self.graphics.set_palette(title1_dat_palette);
        self.graphics.video_loop();

        self.graphics.read_title2_dat();
    }

    fn read_levels_lst(&mut self) {
        // Re-init g_level_list_data
        self.g_level_list_data = [(); K_NUMBER_OF_LEVEL_WITH_PADDING]
            .map(|_| String::from("                           "));
        self.g_level_list_data[K_LAST_LEVEL_INDEX] = String::from("- REPLAY SKIPPED LEVELS!! -");
        self.g_level_list_data[K_LAST_LEVEL_INDEX + 1] =
            String::from("---- UNBELIEVEABLE!!!! ----");

        let path = format!(
            "{}/{}",
            RESSOURCES_PATH, self.demo_manager.g_levels_dat_filename
        );
        let level_lst_file_path = Path::new(&path);
        match level_lst_file_path.try_exists().expect(
            format!(
                "Can't check existence of file {}",
                self.demo_manager.g_levels_dat_filename
            )
            .as_str(),
        ) {
            true => (),
            false => panic!("{:?} doesn't exists", level_lst_file_path.canonicalize()),
        }
        let mut file = File::open(level_lst_file_path).expect(
            format!(
                "Error while opening {}",
                self.demo_manager.g_levels_dat_filename
            )
            .as_str(),
        );

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

            let level_name = format!("{:03} {}", i, String::from_utf8_lossy(&file_data));

            self.g_level_list_data[i] = level_name;

            // TODO ; currently load level name but not data
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

            self.g_hall_of_fame_data[i] = HallOfFameEntry::from(player_data);
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

            self.g_player_list_data[i] = PlayerEntry::from(player_data);
        }
    }

    fn draw_speed_fix_title_and_version(&mut self) {
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            102,
            11,
            1,
            format!("{} VERSION {}", GAME_NAME, VERSION_STRING),
        );
    }

    fn draw_speed_fix_credits(&mut self) // showNewCredits  proc near       ; CODE XREF: start+2ECp
    {
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            60,
            168,
            0xE,
            String::from("VERSIONS 1-4 + 6.X BY HERMAN PERK"),
        );
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            60,
            176,
            0xE,
            String::from("VERSIONS 5.X BY ELMER PRODUCTIONS"),
        );
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            60,
            184,
            0xE,
            String::from("  VERSION 7.X BY SERGIO PADRINO  "),
        );

        self.graphics.video_loop();
        /*
        while (isAnyKeyPressed() == 0
               && isAnyGameControllerButtonPressed() == 0);*/
        // TODO : wait for kpress
    }

    fn draw_text_with_chars6_font_with_opaque_background_if_possible(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        if self.g_is_game_busy {
            return;
        }

        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(dest_x, dest_y, color, text);
    }

    fn draw_text_with_chars6_font_with_transparent_background_if_possible(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        if self.g_is_game_busy {
            return;
        }

        self.graphics
            .draw_text_with_chars6_font_with_transparent_background(dest_x, dest_y, color, text);
    }
    fn default_config(&mut self) {
        self.sounds.activate_combined_sound();
        self.sounds.is_music_enabled = true;
        self.sounds.is_fx_enabled = true;
        self.is_joystick_enabled = false;
    }

    fn read_config(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_CONFIG_FILE_NAME);
        let cfg_file_path = Path::new(&path);
        match cfg_file_path.try_exists().expect(
            format!(
                "Can't check existence of file {}",
                G_HALL_OF_FAME_LST_FILENAME
            )
            .as_str(),
        ) {
            true => (),
            false => {
                self.default_config();
                return;
            } // No player file found
        };
        let mut file = match File::open(cfg_file_path) {
            Ok(file) => file,
            Err(_) => {
                self.default_config();
                return;
            } // Error while reading config file
        };

        let mut buffer = [0_u8; K_CONFIG_DATA_LENGTH];
        match file.read(&mut buffer) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_CONFIG_DATA_LENGTH {
                    self.default_config();
                    return;
                }
            }
            Err(_) => {
                self.default_config();
                return;
            }
        }

        let sound_setting = buffer[0] as char;
        match sound_setting {
            's' => self.sounds.activate_internal_samples_sound(),
            'a' => self.sounds.activate_adlib_sound(),
            'b' => self.sounds.activate_sound_blaster_sound(),
            'r' => self.sounds.activate_roland_sound(),
            'c' => self.sounds.activate_combined_sound(),
            _ => self.sounds.activate_internal_standard_sound(),
        }

        self.is_joystick_enabled = buffer[1] as char == 'j';
        self.sounds.is_music_enabled = buffer[2] as char == 'm';
        self.sounds.is_fx_enabled = buffer[3] as char == 'x';
    }

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
            self.states.g_current_level_state_with_padding[idx].tile = (value & 0xff) as u8;
            self.states.g_current_level_state_with_padding[idx].state = (value >> 8) as u8;
        }

        self.states.g_frame_counter = 0xf000;
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

    fn start_directly_from_level(&mut self, level_number: u8) {
        self.g_is_game_busy = true;
        self.g_should_autoselect_next_level_to_play = true;
        self.prepare_level_data_for_current_player();
        self.draw_player_list();
        self.word_58467 = false;
        self.sounds.play_music_if_needed();
        self.states.g_current_selected_level_index = level_number;
        //restoreLastMouseAreaBitmap();
        self.draw_level_list();
        self.states.g_should_leave_main_menu = false;
        self.byte_5A19B = false;
    }

    fn prepare_level_data_for_current_player(&mut self) {
        let mut current_player_entry =
            &mut (self.g_player_list_data[self.states.g_current_player_index]);

        let current_player_level_state = current_player_entry.level_state;

        // Sets everything to 6 which seems to mean all levels are blocked
        self.states.g_current_player_padded_level_data =
            [K_SKIPPED_LEVEL_ENTRY_COLOR; K_NUMBER_OF_LEVEL_WITH_PADDING];
        for i in 0..K_NUMBER_OF_LEVEL {
            self.states
                .set_g_current_player_level_data(i, K_BLOCKED_LEVEL_ENTRY_COLOR);
        }

        let mut is_first_uncompleted_level = true;

        for i in 0..K_NUMBER_OF_LEVEL {
            if current_player_entry.level_state[i] == PlayerLevelState::PlayerLevelStateSkipped {
                self.states
                    .set_g_current_player_level_data(i, K_SKIPPED_LEVEL_ENTRY_COLOR);
            } else if current_player_entry.level_state[i]
                == PlayerLevelState::PlayerLevelStateCompleted
            // Completed levels
            {
                self.states
                    .set_g_current_player_level_data(i, K_COMPLETED_LEVEL_ENTRY_COLOR);
            } else if current_player_entry.level_state[i]
                == PlayerLevelState::PlayerLevelStateNotCompleted
            // Levels not completed
            {
                if is_first_uncompleted_level {
                    // The first uncompleted is not blocked
                    self.states
                        .set_g_current_player_level_data(i, K_NOT_COMPLETED_LEVEL_ENTRY_COLOR);
                } else {
                    // The rest uncompleted levels are blocked
                    self.states
                        .set_g_current_player_level_data(i, K_BLOCKED_LEVEL_ENTRY_COLOR);
                }
                is_first_uncompleted_level = false;
            }
        }

        let mut has_completed_all_levels: bool = true;
        let mut next_level_to_play: u8 = 1;

        // Looks for the first uncompleted level
        for i in 0..K_NUMBER_OF_LEVEL {
            if current_player_entry.level_state[i] == PlayerLevelState::PlayerLevelStateNotCompleted
            // not completed
            {
                has_completed_all_levels = false;
                break;
            }

            next_level_to_play += 1;
        }

        if has_completed_all_levels {
            next_level_to_play = 1;

            // Looks for the first completed level
            for i in 0..K_NUMBER_OF_LEVEL {
                if current_player_entry.level_state[i] == PlayerLevelState::PlayerLevelStateSkipped
                {
                    has_completed_all_levels = false;
                    break;
                }
                next_level_to_play += 1;
            }
        }

        if has_completed_all_levels {
            if self.g_should_autoselect_next_level_to_play {
                self.states.g_current_selected_level_index = K_LAST_LEVEL_INDEX as u8;
            }

            current_player_entry.next_level_to_play = K_LAST_LEVEL_INDEX as u8;
            return;
        }

        if self.g_should_autoselect_next_level_to_play {
            self.states.g_current_selected_level_index = next_level_to_play;
        }

        if next_level_to_play == 1 {
            if current_player_entry.name == String::from("--------") {
                next_level_to_play = 0;
            }
        }

        current_player_entry.next_level_to_play = next_level_to_play;
    }

    fn draw_player_list(&mut self) {
        let current_player = &self.g_player_list_data[self.states.g_current_player_index];
        self.states.g_player_name = current_player.name.clone();
        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(
                16,
                164,
                6,
                current_player.name.clone(),
            );

        let mut prev_player_name = String::from("");

        if self.states.g_current_player_index <= 0 {
            prev_player_name = String::from("        "); // just 8 spaces :shrug:
        } else {
            prev_player_name = self.g_player_list_data[self.states.g_current_player_index - 1]
                .name
                .clone();
        }

        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(16, 155, 8, prev_player_name);

        let mut next_player_name = String::from("");

        if self.states.g_current_player_index >= K_NUMBER_OF_PLAYERS - 1 {
            next_player_name = String::from("        "); // just 8 spaces :shrug:
        } else {
            next_player_name = self.g_player_list_data[self.states.g_current_player_index + 1]
                .name
                .clone();
        }

        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(16, 173, 8, next_player_name);
        self.draw_current_player_ranking();
    }

    fn draw_current_player_ranking(&mut self) {
        let current_player = &self.g_player_list_data[self.states.g_current_player_index];
        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(
                168,
                93,
                8,
                current_player.name.clone(),
            );

        let time_text = format!(
            "{:03}:{:02}:{:02}",
            current_player.hours, current_player.minutes, current_player.seconds
        );

        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(224, 93, 8, time_text);

        let next_level_text = format!("{:03}", current_player.next_level_to_play);
        self.graphics
            .draw_text_with_chars6_font_with_opaque_background(288, 93, 8, next_level_text);
    }

    fn convert_level_number_to_3_digit_string_with_padding_0(&mut self, value: u8) {
        let s = value.to_string();
        self.g_sp_demo_file_name.replace_range(3..5, s.as_str());
    }

    fn draw_level_list(&mut self) {
        // 01ED:54DE
        let ante_previous_level_data = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize - 2];
        let previous_level_data = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize - 1];
        let current_level_data = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize];

        println!("{:?}", self.g_level_list_data);

        let previous_level_name = match self.states.g_current_selected_level_index {
            0 | 1 => String::new(),
            _ => self.g_level_list_data[(self.states.g_current_selected_level_index as usize - 2)
                * K_LIST_LEVEL_NAME_LENGTH]
                .clone(),
        };
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            155,
            ante_previous_level_data,
            previous_level_name,
        );

        self.states.g_current_level_name = match self.states.g_current_selected_level_index {
            0 => String::new(),
            _ => self.g_level_list_data[(self.states.g_current_selected_level_index as usize - 1)
                * K_LIST_LEVEL_NAME_LENGTH]
                .clone(),
        };
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            164,
            previous_level_data,
            self.states.g_current_level_name.clone(),
        );

        let next_level_name = self.g_level_list_data
            [self.states.g_current_selected_level_index as usize * K_LIST_LEVEL_NAME_LENGTH]
            .clone();
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            173,
            current_level_data,
            next_level_name,
        );
    }

    fn run_main_menu(&mut self) {
        self.g_is_main_menu = true;
        self.g_has_user_interrupted_demo = false;
        self.g_selected_original_demo_level_number = 0;
        self.demo_manager.g_is_sp_demo_available_to_run = 0;
        self.g_automatic_demo_playback_countdown = 4200;
        if self.word_58467 {
            self.graphics.draw_menu_background();
            self.g_should_autoselect_next_level_to_play = false;
            self.prepare_level_data_for_current_player();

            self.draw_menu_title_and_demo_level_result();

            self.graphics.video_loop();
            let palette = self.graphics.get_palette(PaletteType::GamePalette);
            self.graphics.fade_to_palette(palette);
            self.word_58467 = false;
        } else {
            self.byte_59B83 = true;
            self.sub_4C407();
        }

        self.sounds.play_music_if_needed();
        //saveLastMouseAreaBitmap();
        //drawMouseCursor();

        loop {
            (self.g_automatic_demo_playback_countdown, _) =
                self.g_automatic_demo_playback_countdown.overflowing_sub(1);
            if self.g_automatic_demo_playback_countdown == 0 {
                self.handleDemoOptionClick();
            }

            if self.states.g_should_leave_main_menu != false {
                self.states.g_should_leave_main_menu = false;
                break;
            }

            self.graphics.video_loop();
            (self.states.g_frame_counter, _) = self.states.g_frame_counter.overflowing_add(1);

            let mouse_x = self.events.mouse_state().x();
            let mouse_y = self.events.mouse_state().y();
            let mouse_button_status = self
                .events
                .mouse_state()
                .is_mouse_button_pressed(MouseButton::Left)
                as u8
                | ((self
                    .events
                    .mouse_state()
                    .is_mouse_button_pressed(MouseButton::Right) as u8)
                    << 1);

            self.mouse.g_mouse_button_status = mouse_button_status;
            if self.mouse.g_mouse_x != mouse_x || self.mouse.g_mouse_y != mouse_y {
                self.g_automatic_demo_playback_countdown = 4200;
            }

            self.mouse.g_mouse_x = mouse_x;
            self.mouse.g_mouse_y = mouse_y;
            //restoreLastMouseAreaBitmap();
            //saveLastMouseAreaBitmap();
            //drawMouseCursor();
            self.draw_main_menum_button_borders();
            self.update_user_input();
            if self.button_states.gPlayerListDownButtonPressed != false
                || self.button_states.gPlayerListUpButtonPressed != false
            {
                self.button_states.gPlayerListButtonPressed = true;
            }

            self.button_states.gPlayerListDownButtonPressed = false;
            self.button_states.gPlayerListUpButtonPressed = false;
            if self.button_states.gRankingListDownButtonPressed != false
                || self.button_states.gRankingListUpButtonPressed != false
            {
                self.button_states.gRankingListButtonPressed = true;
            }

            self.button_states.gRankingListDownButtonPressed = false;
            self.button_states.gRankingListUpButtonPressed = false;
            if self.button_states.gLevelListDownButtonPressed != false
                || self.button_states.gLevelListUpButtonPressed != false
            {
                self.button_states.gLevelListButtonPressed = true;
            }

            self.button_states.gLevelListDownButtonPressed = false;
            self.button_states.gLevelListUpButtonPressed = false;
            if self.keyboard.gCurrentUserInput as u8 > K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET as u8
            // || isStartButtonPressed() TODO : handle game controller
            {
                self.handleOkButtonClick();
            } else if self.keyboard.gIsF1KeyPressed {
                self.play_demo(0);
            } else if self.keyboard.gIsF2KeyPressed {
                self.play_demo(1);
            } else if self.keyboard.gIsF3KeyPressed {
                self.play_demo(2);
            } else if self.keyboard.gIsF4KeyPressed {
                self.play_demo(3);
            } else if self.keyboard.gIsF5KeyPressed {
                self.play_demo(4);
            } else if self.keyboard.gIsF6KeyPressed {
                self.play_demo(5);
            } else if self.keyboard.gIsF7KeyPressed {
                self.play_demo(6);
            } else if self.keyboard.gIsF8KeyPressed {
                self.play_demo(7);
            } else if self.keyboard.gIsF9KeyPressed {
                self.play_demo(8);
            } else if self.keyboard.gIsF10KeyPressed {
                self.play_demo(9);
            } else if self.keyboard.gIsNumpadDividePressed
                && self.demo_manager.demo_file_name.len() != 0
                && self.demo_manager.file_is_demo
            {
                self.demo_manager.g_is_sp_demo_available_to_run = 1;
                self.play_demo(0);
            } else if self.keyboard.gIsF12KeyPressed && self.demo_manager.demo_file_name.len() != 0
            {
                self.demo_manager.g_is_sp_demo_available_to_run = 1;
                self.states.g_should_leave_main_menu = true;
                self.g_is_playing_demo = false;
                self.states.g_should_update_total_level_time = 0;
                self.states.g_has_user_cheated = true;
                //prepareDemoRecordingFilename();
                // This adds dashes to the level name or something?
                self.demo_manager
                    .g_sp_demo_filename
                    .replace_range(3..6, "---");
                continue;
            }
            if self.mouse.g_mouse_button_status == MOUSE_BUTTON_RIGHT
            // Right button -> exit game
            {
                self.g_should_exit_game = true;
                break;
            } //else if self.keyboard.gIsEscapeKeyPressed
              // ||  getGameControllerButtonBack() TODO implement controller
              /*{
                  runAdvancedOptionsRootMenu();
              } else if (isRotateLevelSetAscendingButtonPressed()) {
                  throttledRotateLevelSet(0);
                  continue; // This allows the throttling effect to act
              } else if (isRotateLevelSetDescendingButtonPressed()) {
                  throttledRotateLevelSet(1);
                  continue; // This allows the throttling effect to act
              }*/
            if self.g_should_exit_game {
                break;
            }

            if self.mouse.g_mouse_button_status == MOUSE_BUTTON_LEFT {
                self.g_automatic_demo_playback_countdown = 4200;

                for i in 0..K_NUMBER_OF_MAIN_MENU_BUTTONS {
                    let button_descriptor = &K_MAIN_MENU_BUTTON_DESCRIPTORS[i];

                    //checkmousecoords:
                    if self.mouse.g_mouse_x >= button_descriptor.start_x
                        && self.mouse.g_mouse_y >= button_descriptor.start_y
                        && self.mouse.g_mouse_x <= button_descriptor.end_x
                        && self.mouse.g_mouse_y <= button_descriptor.end_y
                    {
                        (button_descriptor.callback)(self);
                        break;
                    }
                }
            } else {
                // Reset throttle counters
                self.g_level_list_throttle_current_counter = 0x10;
                self.g_level_list_throttle_next_counter = 0;
                self.g_player_list_throttle_current_counter = 0x10;
                self.g_player_list_throttle_next_counter = 0;
                self.g_ranking_list_throttle_current_counter = 0x10;
                self.g_ranking_list_throttle_next_counter = 0;
                self.g_level_set_rotation_throttle_current_counter = 0x10;
                self.g_level_set_rotation_throttle_next_counter = 0;
            }
        }

        self.g_is_main_menu = false;
        // savePlayerListData(); TODO : save !
        // saveHallOfFameData();
    }

    fn draw_menu_title_and_demo_level_result(&mut self) {
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            168,
            127,
            4,
            String::from("  WELCOME TO SUPAPLEX  "),
        );
        self.draw_player_list();
        self.draw_level_list();
        self.draw_hall_of_fame();
        self.draw_rankings();
        if !self.byte_59B83 {
            return;
        }
        self.byte_59B83 = false;

        let mut message = String::new();
        if !self.byte_5A19B {
            if !self.states.g_is_level_started_as_demo {
                message = String::from("     LEVEL FAILED      ");
            } else {
                message = String::from("      DEMO FAILED      ");
            }
        } else {
            if !self.states.g_is_level_started_as_demo {
                message = String::from("   LEVEL SUCCESSFUL    ");
            } else {
                message = String::from("    DEMO SUCCESSFUL    ");
            }
        }
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(168, 127, 4, message);
        self.byte_5A19B = false;
    }

    fn draw_hall_of_fame(&mut self) {
        for i in 0..K_NUMBER_OF_HALL_OF_FAME_ENTRIES {
            let entry = self.g_hall_of_fame_data[i].clone();

            let text = format!(
                "{:indent$} {:03}:{:02}:{:02}",
                entry.player_name,
                entry.hours,
                entry.minutes,
                entry.seconds,
                indent = 8
            );

            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                184,
                28 + i * 9,
                8,
                text,
            );
        }
    }

    fn draw_rankings(&mut self) {
        self.prepare_ranking_text_entries();

        const K_DISTANCE_BETWEEN_LINES: u8 = 9;

        for i in 0..5 {
            let y: u8 = (110_i32 + K_DISTANCE_BETWEEN_LINES as i32 * (i - 2)) as u8;
            let color: u8 = if i == 2 { 6 } else { 8 };
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                8,
                y as usize,
                color,
                self.g_ranking_text_entries[self.byte_58D46 as usize + i as usize].clone(),
            );
        }

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            110,
            6,
            format!("{:02}", self.byte_58D46),
        );
    }

    fn prepare_ranking_text_entries(&mut self) {
        struct RankingEntry {
            player_index: u8,
            next_level_to_play: u8,
            hours: u8,
            minutes: u8,
            seconds: u8,
        }

        let ranking_entries = (0..20)
            .map(|i| RankingEntry {
                hours: self.g_player_list_data[i].hours,
                minutes: self.g_player_list_data[i].minutes,
                seconds: self.g_player_list_data[i].seconds,
                next_level_to_play: self.g_player_list_data[i].next_level_to_play,
                player_index: i as u8,
            })
            .collect::<Vec<RankingEntry>>();

        let mut number_of_changes: u8 = 0;

        loop {
            number_of_changes = 0;

            for i in 0..(K_NUMBER_OF_PLAYERS - 1) {
                let total_seconds = ranking_entries[i].hours as u32 * 3600
                    + ranking_entries[i].minutes as u32 * 60
                    + ranking_entries[i].seconds as u32;
                let next_total_seconds = ranking_entries[i].hours as u32 * 3600
                    + ranking_entries[i].minutes as u32 * 60
                    + ranking_entries[i].seconds as u32;

                if ranking_entries[i + 1].next_level_to_play > ranking_entries[i].next_level_to_play
                    || (ranking_entries[i + 1].next_level_to_play
                        == ranking_entries[i].next_level_to_play
                        && next_total_seconds > total_seconds)
                {
                    number_of_changes += 1;
                }
            }
            if number_of_changes == 0 {
                break;
            }
        }

        for i in 0..20 {
            if ranking_entries[i].player_index == self.states.g_current_player_index as u8 {
                self.byte_58D47 = i as u8;
            }
        }

        for i in 0..20 {
            self.g_ranking_text_entries[i + 2] = format!(
                "{} {:indent$} {:03}:{:02}:{:02}",
                if ranking_entries[i].next_level_to_play == 0x71 {
                    999
                } else {
                    ranking_entries[i].next_level_to_play as u32
                },
                self.g_player_list_data[ranking_entries[i].player_index as usize].name,
                ranking_entries[i].hours,
                ranking_entries[i].minutes,
                ranking_entries[i].seconds,
                indent = 8
            )
        }
    }

    fn sub_4C407(&mut self) {
        if self.g_level_failed {
            self.g_level_failed = false;
            self.draw_failed_level_result_screen();
            self.graphics.draw_menu_background();

            self.g_should_autoselect_next_level_to_play = false;

            self.prepare_level_data_for_current_player();
            self.draw_menu_title_and_demo_level_result();
            let palette = self.graphics.get_palette(PaletteType::GamePalette);
            self.graphics.fade_to_palette(palette);

            self.graphics.video_loop();

            // This will prevent to leave traces of the options menu
            // area in the main menu.
            //saveLastMouseAreaBitmap();
        } else {
            self.scroll_left_to_main_menu();
        }
    }

    fn draw_failed_level_result_screen(&mut self) {
        self.graphics.set_palette(G_BLACK_PALETTE);
        self.graphics.draw_back_background();

        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            128,
            60,
            0xF,
            String::from("HARD LUCK!"),
        );
        if self.states.g_number_of_remaining_infotrons == 0 {
            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                40,
                80,
                0xF,
                String::from("YOU COMPLETED ALL THE NECESSARY INFOTRONS"),
            );
            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                72,
                100,
                0xF,
                String::from("BUT FAILED TO REACH THE EXIT"),
            );
        } else {
            let collected_infotrons = self.states.g_total_number_of_infotrons
                - self.states.g_number_of_remaining_infotrons;

            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                40,
                80,
                0xF,
                format!(
                    "YOU HAVE COLLECTED {:03} OUT OF THE {:03}",
                    collected_infotrons, self.states.g_total_number_of_infotrons
                ),
            );
            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                104,
                100,
                0xF,
                String::from("INFOTRONS NEEDED"),
            );
        }

        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            72,
            120,
            0xF,
            String::from("WHY NOT GIVE IT ANOTHER TRY?"),
        );

        self.graphics.video_loop();
        let palette = self
            .graphics
            .get_palette(PaletteType::InformationScreenPalette);
        self.graphics.set_palette(palette);

        if !self.g_should_exit_game {
            self.wait_for_key_press_or_mouse_click();
        }

        self.graphics.set_palette(G_BLACK_PALETTE);
    }

    fn scroll_left_to_main_menu(&mut self) {
        let current_screen_pixels = self.graphics.video.borrow_mut().get_screen_pixels();
        let menu_screen_pixels = self.graphics.video.borrow_mut().get_screen_pixels();

        self.graphics.draw_menu_background();
        self.g_should_autoselect_next_level_to_play = false;

        self.prepare_level_data_for_current_player();
        self.draw_menu_title_and_demo_level_result();

        let menu_screen_pixels = self.graphics.video.borrow_mut().get_screen_pixels();

        const K_NUMBER_OF_STEPS: u32 = 80;

        let mut k_animation_duration: u32 = K_NUMBER_OF_STEPS * 1000 / 70; // ~571 ms
        let mut animation_time = 0;

        self.graphics.start_tracking_tender_delta_time();

        // Draws the current scroll animation step
        while animation_time < k_animation_duration {
            animation_time += self.graphics.update_render_delta_time();
            animation_time = std::cmp::min(animation_time, k_animation_duration);

            let animation_factor = animation_time as f64 / k_animation_duration as f64;

            let limit_from_left = animation_factor * K_SCREEN_WIDTH as f64;
            let limit_from_right = (animation_factor - limit_from_left) as usize;
            let limit_from_left = limit_from_left as usize;

            for y in 0..K_SCREEN_HEIGHT {
                // Main menu side
                for x in 0..(K_SCREEN_WIDTH - limit_from_right) {
                    let color =
                        menu_screen_pixels[y * K_SCREEN_WIDTH + x + limit_from_right as usize];
                    self.graphics
                        .video
                        .borrow_mut()
                        .set_pixel(y * K_SCREEN_WIDTH + x, color);
                }

                // GFX background side
                for x in limit_from_left..K_SCREEN_WIDTH {
                    let color =
                        current_screen_pixels[y * K_SCREEN_WIDTH + x - limit_from_left as usize];
                    self.graphics
                        .video
                        .borrow_mut()
                        .set_pixel(y * K_SCREEN_WIDTH + x, color);
                }
            }

            self.graphics.video_loop();
        }
        // This will prevent to leave traces of the options menu
        // area in the main menu.
        //saveLastMouseAreaBitmap();
    }

    fn draw_main_menum_button_borders(&mut self) {
        let mut color = 0;

        if self.button_states.gPlayerListButtonPressed != false {
            if self.button_states.gPlayerListUpButtonPressed == false {
                color = 7;
            } else {
                color = 0xD; // 13
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[0], color);
            if self.button_states.gPlayerListUpButtonPressed == false {
                color = 0xD; // 13
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[1], color);
            if self.button_states.gPlayerListDownButtonPressed == false {
                color = 7;
            } else {
                color = 0xD; // 13
            }
            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[2], color);
            if self.button_states.gPlayerListDownButtonPressed == false {
                color = 0xD; // 13
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[3], color);
            self.button_states.gPlayerListButtonPressed = false;
        }

        if self.button_states.gRankingListButtonPressed != false {
            if self.button_states.gRankingListUpButtonPressed == false {
                color = 7;
            } else {
                color = 0xD; // 13
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[4], color);
            if self.button_states.gRankingListUpButtonPressed == false {
                color = 0xD;
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[5], color);
            if self.button_states.gRankingListDownButtonPressed == false {
                color = 7;
            } else {
                color = 0xD;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[6], color);
            if self.button_states.gRankingListDownButtonPressed == false {
                color = 0xD;
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[7], color);
            self.button_states.gRankingListButtonPressed = false;
        }

        if self.button_states.gLevelListButtonPressed == false {
            return;
        }
        if self.button_states.gLevelListUpButtonPressed == false {
            color = 7;
        } else {
            color = 0xD;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[8], color);
        if self.button_states.gLevelListUpButtonPressed == false {
            color = 0xD;
        } else {
            color = 7;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[9], color);
        if self.button_states.gLevelListDownButtonPressed == false {
            color = 7;
        } else {
            color = 0xD;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[10], color);
        if self.button_states.gLevelListDownButtonPressed == false {
            color = 0xD;
        } else {
            color = 7;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[11], color);
        self.button_states.gLevelListButtonPressed = false;
    }

    fn handleNewPlayerOptionClick(&mut self) {}
    fn handleDeletePlayerOptionClick(&mut self) {}
    fn handleSkipLevelOptionClick(&mut self) {}
    fn handleStatisticsOptionClick(&mut self) {}
    fn handleGfxTutorOptionClick(&mut self) {}
    fn handleDemoOptionClick(&mut self) {}
    fn handleControlsOptionClick(&mut self) {}
    fn handleRankingListScrollUp(&mut self) {}
    fn handleRankingListScrollDown(&mut self) {}
    fn handleOkButtonClick(&mut self) {}
    fn handleFloppyDiskButtonClick(&mut self) {}
    fn handlePlayerListScrollUp(&mut self) {}
    fn handlePlayerListScrollDown(&mut self) {}
    fn handlePlayerListClick(&mut self) {}
    fn handleLevelListScrollUp(&mut self) {}
    fn handleLevelListScrollDown(&mut self) {}
    fn handleLevelCreditsClick(&mut self) {}

    fn fun_level(&mut self) {
        // TODO : implement function
    }

    fn update_user_input(&mut self) {
        let mut directionKeyWasPressed = 0;

        self.keyboard.gCurrentUserInput = UserInput::UserInputNone;

        if self.isUpButtonPressed() {
            self.keyboard.gCurrentUserInput = UserInput::UserInputUp;
            directionKeyWasPressed = 1;
        }

        if self.isLeftButtonPressed() {
            self.keyboard.gCurrentUserInput = UserInput::UserInputLeft;
            directionKeyWasPressed = 1;
        }

        if self.isDownButtonPressed() {
            self.keyboard.gCurrentUserInput = UserInput::UserInputDown;
            directionKeyWasPressed = 1;
        }

        if self.isRightButtonPressed() {
            self.keyboard.gCurrentUserInput = UserInput::UserInputRight;
            directionKeyWasPressed = 1;
        }

        if self.isActionButtonPressed() {
            if directionKeyWasPressed == 1 {
                self.keyboard.gCurrentUserInput += K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET;
            } else {
                self.keyboard.gCurrentUserInput = UserInput::UserInputSpaceOnly;
            }
        }
    }

    fn isActionButtonPressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Space)
    }

    fn isUpButtonPressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Up)
    }

    fn isDownButtonPressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Down)
    }

    fn isLeftButtonPressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Left)
    }

    fn isRightButtonPressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Right)
    }
}
