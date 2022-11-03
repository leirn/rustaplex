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
pub mod level;
mod sounds;
mod utils;
pub mod video;

use demo::DemoManager;
use game_states::GameStates;
use globals::*;
use graphics::{Graphics, G_TITLE1_PALETTE_DATA, G_TITLE2_PALETTE_DATA, G_TITLE_PALETTE_DATA};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sounds::Sounds;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use video::Video;

use self::graphics::G_BLACK_PALETTE;

enum UserInput {
    UserInputNone = 0,
    UserInputUp = 1,
    UserInputLeft = 2,
    UserInputDown = 3,
    UserInputRight = 4,
    UserInputSpaceUp = 5,
    UserInputSpaceLeft = 6,
    UserInputSpaceDown = 7,
    UserInputSpaceRight = 8,
    UserInputSpaceOnly = 9,
}

pub struct Game<'a> {
    sounds: Sounds<'a>,
    graphics: Graphics<'a>,
    video: Rc<RefCell<Video<'a>>>,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    g_random_generator_seed: u16,
    g_level_list_data: [String; K_NUMBER_OF_LEVEL_WITH_PADDING],
    g_player_list_data: [PlayerEntry; K_NUMBER_OF_PLAYERS],
    g_hall_of_fame_data: [HallOfFameEntry; K_NUMBER_OF_HALL_OF_FAME_ENTRIES],
    g_is_game_busy: bool,
    is_joystick_enabled: bool,
    demo_manager: DemoManager,
    states: GameStates,
}

impl Game<'_> {
    pub fn new() -> Game<'static> {
        let sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        let video = Rc::new(RefCell::new(Video::init(sdl_context.clone())));
        Game {
            sounds: Sounds::new(sdl_context.clone()),
            video: video.clone(),
            graphics: Graphics::init(video.clone(), sdl_context.clone()),
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
        while should_quit_the_game {
            self.demo_manager.prepare_demo_recording_filename();
            let level_number_forced_to_load = 0_u8;

            if self.demo_manager.g_is_sp_demo_available_to_run == 2 {
                self.demo_manager.g_is_sp_demo_available_to_run = 1;
                if self.demo_manager.file_is_demo {
                    self.play_demo(0);
                } else {
                    self.demo_manager.g_is_playing_demo = false;
                }

                /*
                gShouldUpdateTotalLevelTime = 0;
                gHasUserCheated = 1;
                memcpy(&gSPDemoFileName[3], "---", 3);
                startDirectlyFromLevel(1);
                continue;
                */
            } else {
                /*
                levelNumberForcedToLoad = gIsForcedLevel;
                gIsForcedLevel = 0;
                gIsPlayingDemo = 0;

                if (levelNumberForcedToLoad > 0)
                {
                    convertLevelNumberTo3DigitStringWithPadding0(levelNumberForcedToLoad);
                }
                */
            }
            /*

                    if (levelNumberForcedToLoad > 0)
                    {
            //loc_4701A:              //; CODE XREF: start+3DDj start+433j
                        startDirectlyFromLevel(levelNumberForcedToLoad);
                        continue;
                    }

            //loc_4704B:              //; CODE XREF: start+3EEj start+3F2j
                    if (gShouldStartFromSavedSnapshot != 0)
                    {
            //loc_4701A:              //; CODE XREF: start+3DDj start+433j
                        startDirectlyFromLevel(1);
                        continue;
                    }
                    gHasUserCheated = 0;
                    runMainMenu();
            */

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

            if self.sounds.is_music_enabled == false
            {
                self.sounds.stop_music();
            }

            self.g_is_game_busy = true;
            self.run_level();
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

            if (isMusicEnabled == 0)
            {
                playMusicIfNeeded();
            }*/

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
        let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
        loop {
            for event in event_pump.poll_iter() {
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
        let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
        for event in event_pump.poll_iter() {
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

        /* gRandomGeneratorSeed = gDemoRandomSeeds[demo_index];
            gShouldLeaveMainMenu = 1;
            gIsPlayingDemo = 1;

            let demo_first_index; u16 = gDemos.demoFirstIndices[demo_index];
            if (demo_first_index == 0xFFFF)
            {
                gShouldLeaveMainMenu = 0;
                gIsPlayingDemo = 0;
            }

        //loc_4B22F:              ; CODE XREF: playDemo+30j
            gSelectedOriginalDemoLevelNumber = 0;

             demoLevelNumber = gDemos.demoData[demoFirstIndex];
             finalLevelNumber = demoIndex;

            if (demoLevelNumber <= kNumberOfLevels // 111
                && demoLevelNumber != 0)
            {
                finalLevelNumber = demoLevelNumber;
                gSelectedOriginalDemoLevelNumber = (gSelectedOriginalDemoLevelNumber & 0xFF00) | finalLevelNumber; // mov byte ptr gSelectedOriginalDemoLevelNumber, al
            }

        //loc_4B248:              ; CODE XREF: playDemo+4Bj
        //                ; playDemo+4Fj
            gDemoIndexOrDemoLevelNumber = finalLevelNumber;

            demoFirstIndex++; // To skip the level number
            gDemoCurrentInputIndex = demoFirstIndex;
            word_5A33C = demoFirstIndex;
            gDemoCurrentInput = UserInputNone;
            gDemoCurrentInputRepeatCounter = 1;*/
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
}
