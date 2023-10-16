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
pub mod gamecontroller;
pub mod globals;
pub mod graphics;
mod input;
mod keyboard;
pub mod level;
mod mouse;
mod sounds;
mod utils;
pub mod video;

use crate::game::button_borders::{
    K_NUMBER_OF_OPTIONS_MENU_BUTTONS, K_OPTIONS_MENU_BUTTON_DESCRIPTORS,
};
use crate::game::graphics::{
    DestinationSurface, K_FIXED_BITMAP_WIDTH, K_LEVEL_BITMAP_HEIGHT, K_LEVEL_BITMAP_WIDTH,
    K_LEVEL_EDGE_SIZE, K_TILE_SIZE,
};

use self::button_borders::{
    ButtonBorderLineDescriptor, ButtonBorderLineType, K_OPTIONS_MENU_BORDERS,
};
use self::graphics::{BitmapType, K_PANEL_BITMAP_HEIGHT, K_SCREEN_HEIGHT, K_SCREEN_WIDTH};
use self::input::Input;
use self::level::LevelManager;
use self::sounds::SoundType;
use button_borders::{
    ButtonStatus, K_MAIN_MENU_BUTTON_BORDERS, K_MAIN_MENU_BUTTON_DESCRIPTORS,
    K_NUMBER_OF_MAIN_MENU_BUTTONS,
};
use demo::DemoManager;
use game_states::GameStates;
use globals::*;
use graphics::{Graphics, PaletteType, K_FULL_SCREEN_FRAMEBUFFER_LENGTH, K_MOVING_BITMAP_WIDTH};
use keyboard::{Keys, UserInput, K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET};
use mouse::{Mouse, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_RIGHT};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::sys::SDL_EventType;
use sdl2::EventPump;
use sounds::Sounds;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use video::Video;

struct Files {
    player: String,
    level_list: String,
    hall_of_fame: String,
    demo_bin: String,
    savegame: String,
}

impl Files {
    pub fn new() -> Files {
        Files {
            player: G_PLAYERS_LST_FILENAME.to_string(),
            level_list: G_LEVELS_LST_FILENAME.to_string(),
            hall_of_fame: G_HALL_OF_FAME_LST_FILENAME.to_string(),
            demo_bin: "DEMO0.BIN".to_string(),
            savegame: G_SAVEGAME_SAV_FILENAME.to_string(),
        }
    }

    pub fn change_suffix(&mut self, suffix: &str) {
        let mut current_suffix = suffix;
        if current_suffix == "AT" {
            current_suffix = "ST";
        }
        self.level_list = format!("{}{}", self.level_list.get(0..7).unwrap(), current_suffix);
        self.player = format!("{}{}", self.player.get(0..8).unwrap(), current_suffix);
        self.hall_of_fame = format!(
            "{}{}",
            self.hall_of_fame.get(0..0xa).unwrap(),
            current_suffix
        );

        if current_suffix == "ST" {
            current_suffix == "IN";
        }

        self.demo_bin = format!("{}{}", self.demo_bin.get(0..7).unwrap(), current_suffix);

        if current_suffix == "IN" {
            current_suffix == "AV";
        }

        self.savegame = format!("{}{}", self.savegame.get(0..0xa).unwrap(), current_suffix);
    }
}

pub struct Game<'a> {
    files: Files,
    sounds: Sounds<'a>,
    graphics: Graphics<'a>,
    video: Rc<RefCell<Video<'a>>>,
    events: EventPump,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    g_random_generator_seed: u16,
    g_player_list_data: Box<[Box<PlayerEntry>; K_NUMBER_OF_PLAYERS]>,
    g_hall_of_fame_data: Box<[Box<HallOfFameEntry>; K_NUMBER_OF_HALL_OF_FAME_ENTRIES]>,
    g_is_game_busy: bool,
    g_is_debug_mode_enabled: bool,
    is_joystick_enabled: bool,
    demo_manager: Box<DemoManager>,
    level_manager: Box<LevelManager>,
    states: GameStates,
    g_has_user_cheated: bool,
    g_should_autoselect_next_level_to_play: bool,
    g_is_forced_level: u8,
    g_is_forced_cheat_mode: bool,
    g_is_playing_demo: bool,
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
    current_ranking_index: u8, // byte_58D46
    byte_59B83: bool,
    byte_50919: u8,
    should_quit_option_menu: bool, // word_58463 in open-supaplex
    g_level_failed: bool,
    button_states: ButtonStatus,
    keyboard: Rc<RefCell<Keys>>,
    input: Input,
    mouse: Mouse,
    g_level_list_throttle_current_counter: u16,
    g_level_list_throttle_next_counter: u16,
    g_player_list_throttle_current_counter: u16,
    g_player_list_throttle_next_counter: u16,
    g_ranking_list_throttle_current_counter: u16,
    g_ranking_list_throttle_next_counter: u16,
    g_level_set_rotation_throttle_current_counter: u16,
    g_level_set_rotation_throttle_next_counter: u16,
}

impl Game<'_> {
    pub fn new() -> Game<'static> {
        let sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        let video = Rc::new(RefCell::new(Video::init(sdl_context.clone())));

        let events = sdl_context.borrow_mut().event_pump().unwrap();

        let keyboard = Rc::new(RefCell::new(Keys::default()));
        let input = Input::new(keyboard.clone());

        Game {
            files: Files::new(),
            sounds: Sounds::new(sdl_context.clone()),
            video: video.clone(),
            graphics: Graphics::init(video.clone(), sdl_context.clone()),
            events: events,
            sdl_context: sdl_context,
            g_random_generator_seed: 0,
            g_player_list_data: Box::new([(); K_NUMBER_OF_PLAYERS].map(|_|Box::new( PlayerEntry::new()))),
            g_hall_of_fame_data: Box::new([(); K_NUMBER_OF_HALL_OF_FAME_ENTRIES]
                .map(|_| Box::new(HallOfFameEntry::new()))),
            g_is_game_busy: false,
            g_is_debug_mode_enabled: false,
            is_joystick_enabled: false,
            demo_manager: Box::new(DemoManager::new()),
            level_manager: Box::new(LevelManager::new()),
            states: GameStates::new(),
            g_has_user_cheated: false,
            g_should_autoselect_next_level_to_play: false,
            g_is_forced_level: 0,
            g_is_forced_cheat_mode: false,
            g_is_playing_demo: false,
            g_should_start_from_saved_snapshot: false,
            word_58467: true,
            byte_5A19B: false,
            byte_50919: 0,
            should_quit_option_menu: false,
            g_is_main_menu: false,
            g_has_user_interrupted_demo: false,
            g_selected_original_demo_level_number: 0,
            g_automatic_demo_playback_countdown: 0,
            g_should_exit_game: false,
            g_ranking_text_entries: [(); K_NUMBER_OF_PLAYERS + 4].map(|_| String::new()),
            byte_58D47: 0,
            current_ranking_index: 0,
            byte_59B83: false,
            g_level_failed: false,
            button_states: ButtonStatus::default(),
            keyboard: keyboard,
            input: input,
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

        log::info!("Starting game engine");

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
            self.graphics.fade_to_palette(PaletteType::Title);

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
        self.graphics.fade_to_palette(PaletteType::Black);
    }

    fn run(&mut self) {
        log::info!("Starting game main loop");

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
            /* TODO later since only in second cycle */
            // self.read_levels(); already done in read_level_lst
            self.graphics.fade_to_palette(PaletteType::Black);
            self.g_is_game_busy = false;
            self.draw_player_list();
            self.initialize_game_info();
            self.draw_fixed_level();
            self.draw_game_panel(); // 01ED:0311
            let number_of_infotrons: u16 = self.convert_to_easy_tiles();
            self.reset_number_of_infotrons(number_of_infotrons);
            self.find_murphy();
            self.states.g_current_panel_height = K_PANEL_BITMAP_HEIGHT;
            self.graphics
                .draw_current_level_viewport(self.states.g_current_panel_height); // Added by open-supaplex
            self.graphics.fade_to_palette(PaletteType::GamePalette); // At this point the screen fades in and shows the game

            if self.sounds.is_music_enabled == false {
                self.sounds.stop_music();
            }

            self.g_is_game_busy = true;
            self.run_level();

            self.demo_manager.g_is_sp_demo_available_to_run = 0;
            if self.g_should_exit_game {
                break; // goto loc_47067;
            }
            if self.graphics.fast_mode != FastModeType::None {
                break;
            }
            //slideDownGameDash(); // 01ED:0351
            // if self.byte_59B71 != 0 {
            //     self.load_murphy_sprites();
            // }

            self.g_is_game_busy = false;
            if self.g_should_exit_game {
                break; // goto loc_47067;
            }

            if self.sounds.is_music_enabled {
                self.sounds.play_music_if_needed();
            }

            self.graphics.video_loop();

            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. } => should_quit_the_game = true,
                    /*Event::KeyUp {
                        keycode: Some(Keycode::F),
                        ..
                    } => self.video.borrow_mut().toggle_fullscreen(),
                    Event::Window { win_event, .. } => {
                        if let WindowEvent::Resized(_w, _h) = win_event {
                            //handle the resize event
                            println!("Window resized Event received");
                            self.video.borrow_mut().update_window_viewport();
                        }
                    }*/
                    _ => (),
                }
            }
        }
    }

    fn run_level(&mut self) {
        /*
            if (gIsPlayingDemo == 0)
            {
                gIsLevelStartedAsDemo = 0;
                gLevelFailed = 1;
            }
            else
            {
                gIsLevelStartedAsDemo = 1;
                gLevelFailed = 0;
            }

            if (gDemoRecordingJustStarted == 1)
            {
                gDemoRecordingJustStarted = 0;
                drawGameTime();

                do
                {
                    int9handler(1);
                }
                while (areAnyF1ToF10KeysPressed());

                initializeGameInfo();
                if (isMusicEnabled == 0)
                {
                    stopMusic();
                }

                gIsLevelStartedAsDemo = 0;
                gLevelFailed = 1;
            }

            gPlantedRedDiskCountdown = 0;
            byte_5A323 = 0;

            do
            {
                handleGameIterationStarted();

                int9handler(0);

                uint16_t mouseButtonsStatus;

                getMouseStatus(NULL, NULL, &mouseButtonsStatus);

                if (gIsDebugModeEnabled != 0)
                {
                    if (gToggleFancyEasyTilesThrottleCounter != 0)
                    {
                        gToggleFancyEasyTilesThrottleCounter--;
                    }

                    if (gIsEnterPressed == 0
                        && mouseButtonsStatus == MouseButtonLeft //cmp bx, 1
                        && gToggleFancyEasyTilesThrottleCounter == 0)
                    {
                        gToggleFancyEasyTilesThrottleCounter = 0xA;
                        restoreOriginalFancyTiles(); // 01ED:1EFF
                        drawFixedLevel();
                        convertToEasyTiles();
                    }
                }

                handleGameUserInput(); // 01ED:1F08
                if (gDemoRecordingJustStarted == 1)
                {
                    // Restart the demo
                    gDemoRecordingJustStarted = 0;
                    drawGameTime();

                    do
                    {
        //isFunctionKey:
                        int9handler(1);
                    }
                    while (areAnyF1ToF10KeysPressed());

        //notFunctionKey:
                    initializeGameInfo();
                    if (isMusicEnabled == 0)
                    {
                        stopMusic();
                    }

                    gIsLevelStartedAsDemo = 0;
                    gLevelFailed = 1;

                    gPlantedRedDiskCountdown = 0;
                    byte_5A323 = 0;

                    continue;

                    // All the code in this "if" is equivalent to "jmp loc_48ADF"
                }

                if (gIsFlashingBackgroundModeEnabled != 0)
                {
                    replaceCurrentPaletteColor(0, (Color) { 0x35, 0x35, 0x35 });
                }

        //noFlashing:
                updateMovingObjects(); // 01ED:1F28
                if (gIsFlashingBackgroundModeEnabled != 0)
                {
                    replaceCurrentPaletteColor(0, (Color) { 0x21, 0x21, 0x21 });
                }

        //noFlashing2:
                drawGameTime();
                clearAdditionalInfoInGamePanelIfNeeded();
                if (gIsFlashingBackgroundModeEnabled != 0)
                {
                    replaceCurrentPaletteColor(0, (Color) { 0x2d, 0x21, 0x0f });
                }

                updatePlantedRedDisk();
                updateExplosionTimers();
                updateScrollOffset();

                if (gIsFlashingBackgroundModeEnabled != 0)
                {
                    replaceCurrentPaletteColor(0, (Color) { 0x3f, 0x3f, 0x3f });
                }

                drawCurrentLevelViewport(gCurrentPanelHeight); // Added by open-supaplex
                if (gFastMode != FastModeTypeUltra)
                {
                    videoLoop(); // 01ED:2142
                }
                handleGameIterationFinished();

                if (gDebugExtraRenderDelay > 1)
                {
                    playBaseSound();
                }

                // Extra delays in debug mode
                for (int i = 1; i < gDebugExtraRenderDelay; ++i)
                {
                    if (gFastMode == FastModeTypeNone)
                    {
                        videoLoop(); // 01ED:2160
                    }

                    handleGameUserInput();
                }

                if (gIsFlashingBackgroundModeEnabled != 0)
                {
                    replaceCurrentPaletteColor(0, (Color) { 0, 0, 0 });
                }

                if (gShouldExitGame != 0)
                {
                    break;
                }
                gFrameCounter++;
                if (gShouldExitLevel == 1)
                {
                    break;
                }
                if (gQuitLevelCountdown == 0) // 01ED:218D
                {
                    continue;
                }

                gQuitLevelCountdown--;
                if (gQuitLevelCountdown == 0)
                {
                    break;
                }
            }
            while(1);

            if (gIsRecordingDemo != 0)
            {
                stopRecordingDemo();
            }

            uint8_t userDidNotCheat = (gHasUserCheated == 0);
            gHasUserCheated = 0;
            if (userDidNotCheat
                && gShouldUpdateTotalLevelTime != 0
                && byte_5A323 == 0)
            {
                addCurrentGameTimeToPlayer();
            }

            gIsMoveScrollModeEnabled = 0;
            gAdditionalScrollOffsetX = 0;
            gAdditionalScrollOffsetY = 0;
            gIsFlashingBackgroundModeEnabled = 0;
            gDebugExtraRenderDelay = 1;
            replaceCurrentPaletteColor(0, (Color) { 0, 0, 0 }); */
    }

    fn wait_for_key_press_or_mouse_click(&mut self) {
        self.events.pump_events();
        self.sdl_context.borrow_mut().event().unwrap().flush_events(
            SDL_EventType::SDL_KEYDOWN as u32,
            SDL_EventType::SDL_MOUSEWHEEL as u32,
        );
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

        if demo_level_number <= K_NUMBER_OF_LEVELS as u16 && demo_level_number != 0 {
            final_level_number = demo_level_number;
            self.demo_manager.g_selected_original_demo_level_number =
                (self.demo_manager.g_selected_original_demo_level_number & 0xFF00)
                    | final_level_number as usize;
        }
        self.demo_manager.g_demo_index_or_demo_level_number = final_level_number;

        demo_first_index += 1; // To skip the level number
        self.demo_manager.g_demo_current_input_index = demo_first_index;
        self.demo_manager.word_5A33C = demo_first_index;
        self.demo_manager.g_demo_current_input = UserInput::None;
        self.demo_manager.g_demo_current_input_repeater_count = 1;
    }

    fn load_all_ressources(&mut self) {
        self.level_manager.read_levels_lst();
        self.demo_manager.read_demo_files();
        self.read_hall_fame_lst();
        self.read_players_lst();
    }

    fn load_screen_2(&mut self) {
        self.graphics.read_and_render_title1_dat();
        self.graphics.set_palette(PaletteType::Title1);
        self.graphics.video_loop();

        self.graphics.read_title2_dat();
    }

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

            self.g_hall_of_fame_data[i] = Box::new(HallOfFameEntry::from(player_data));
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

            self.g_player_list_data[i] = Box::new(PlayerEntry::from(player_data));
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

    fn generate_random_number(&mut self) -> u16 {
        let mut some_value = self.g_random_generator_seed;
        some_value *= 0x5E5; // 1509
        some_value += 0x31; // '1' or 49
        self.g_random_generator_seed = some_value;
        some_value / 2
    }

    fn initialize_fade_palette(&mut self) {
        self.graphics.set_palette(PaletteType::Black);
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
            Box::new([K_SKIPPED_LEVEL_ENTRY_COLOR; K_NUMBER_OF_LEVEL_WITH_PADDING]);
        for i in 0..K_NUMBER_OF_LEVELS {
            self.states
                .set_g_current_player_level_data(i, K_BLOCKED_LEVEL_ENTRY_COLOR);
        }

        let mut is_first_uncompleted_level = true;

        for i in 0..K_NUMBER_OF_LEVELS {
            if current_player_entry.level_state[i] == PlayerLevelState::Skipped {
                self.states
                    .set_g_current_player_level_data(i, K_SKIPPED_LEVEL_ENTRY_COLOR);
            } else if current_player_entry.level_state[i] == PlayerLevelState::Completed
            // Completed levels
            {
                self.states
                    .set_g_current_player_level_data(i, K_COMPLETED_LEVEL_ENTRY_COLOR);
            } else if current_player_entry.level_state[i] == PlayerLevelState::NotCompleted
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
        for i in 0..K_NUMBER_OF_LEVELS {
            if current_player_entry.level_state[i] == PlayerLevelState::NotCompleted
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
            for i in 0..K_NUMBER_OF_LEVELS {
                if current_player_entry.level_state[i] == PlayerLevelState::Skipped {
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
        log::debug!("convert_level_number_to_3_digit_string_with_padding_0. previous filename = {}, value = {}", self.demo_manager.g_sp_demo_filename, value);
        let s = format!("{:03}", value.to_string());
        self.demo_manager
            .g_sp_demo_filename
            .replace_range(3..6, s.as_str());
    }

    fn draw_level_list(&mut self) {
        // 01ED:54DE
        let ante_previous_level_data = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize - 2];
        let previous_level_data = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize - 1];
        let current_level_data = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize];

        let previous_level_name = match self.states.g_current_selected_level_index {
            0 | 1 => String::from(" ").repeat(27),
            _ => self.level_manager.g_level_list_data
                [self.states.g_current_selected_level_index as usize - 2]
                .name
                .clone(),
        };
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            155,
            ante_previous_level_data,
            previous_level_name,
        );

        self.states.g_current_level_name = match self.states.g_current_selected_level_index {
            0 => String::from(" ").repeat(27),
            _ => self.level_manager.g_level_list_data
                [self.states.g_current_selected_level_index as usize - 1]
                .name
                .clone(),
        };
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            164,
            previous_level_data,
            self.states.g_current_level_name.clone(),
        );

        let next_level_name = self.level_manager.g_level_list_data
            [self.states.g_current_selected_level_index as usize]
            .name
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
            self.graphics.fade_to_palette(PaletteType::GamePalette);
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
                self.handle_demo_option_click();
            }

            if self.states.g_should_leave_main_menu != false {
                self.states.g_should_leave_main_menu = false;
                break;
            }

            self.graphics.video_loop();
            (self.states.g_frame_counter, _) = self.states.g_frame_counter.overflowing_add(1);

            let mouse_status = self.get_mouse_status();

            if self.mouse.x != mouse_status.x || self.mouse.y != mouse_status.y {
                self.g_automatic_demo_playback_countdown = 4200;
            }
            self.mouse = mouse_status;
            //restoreLastMouseAreaBitmap();
            //saveLastMouseAreaBitmap();
            //drawMouseCursor();
            self.draw_main_menum_button_borders();
            self.update_user_input();
            if self.button_states.g_player_list_down_button_pressed != false
                || self.button_states.g_player_list_up_button_pressed != false
            {
                self.button_states.g_player_list_button_pressed = true;
            }

            self.button_states.g_player_list_down_button_pressed = false;
            self.button_states.g_player_list_up_button_pressed = false;
            if self.button_states.g_ranking_list_down_button_pressed != false
                || self.button_states.g_ranking_list_up_button_pressed != false
            {
                self.button_states.g_ranking_list_button_pressed = true;
            }

            self.button_states.g_ranking_list_down_button_pressed = false;
            self.button_states.g_ranking_list_up_button_pressed = false;
            if self.button_states.g_level_list_down_button_pressed != false
                || self.button_states.g_level_list_up_button_pressed != false
            {
                self.button_states.g_level_list_button_pressed = true;
            }

            self.button_states.g_level_list_down_button_pressed = false;
            self.button_states.g_level_list_up_button_pressed = false;

            if self.keyboard.borrow_mut().g_current_user_input as u8
                > K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET as u8
                || self.input.is_start_button_pressed()
            {
                self.handle_ok_button_click();
            } else if self.keyboard.borrow_mut().g_is_f1_key_pressed {
                self.play_demo(0);
            } else if self.keyboard.borrow_mut().g_is_f2_key_pressed {
                self.play_demo(1);
            } else if self.keyboard.borrow_mut().g_is_f3_key_pressed {
                self.play_demo(2);
            } else if self.keyboard.borrow_mut().g_is_f4_key_pressed {
                self.play_demo(3);
            } else if self.keyboard.borrow_mut().g_is_f5_key_pressed {
                self.play_demo(4);
            } else if self.keyboard.borrow_mut().g_is_f6_key_pressed {
                self.play_demo(5);
            } else if self.keyboard.borrow_mut().g_is_f7_key_pressed {
                self.play_demo(6);
            } else if self.keyboard.borrow_mut().g_is_f8_key_pressed {
                self.play_demo(7);
            } else if self.keyboard.borrow_mut().g_is_f9_key_pressed {
                self.play_demo(8);
            } else if self.keyboard.borrow_mut().g_is_f10_key_pressed {
                self.play_demo(9);
            } else if self.keyboard.borrow_mut().g_is_numpad_divide_pressed
                && self.demo_manager.demo_file_name.len() != 0
                && self.demo_manager.file_is_demo
            {
                self.demo_manager.g_is_sp_demo_available_to_run = 1;
                self.play_demo(0);
            } else if self.keyboard.borrow_mut().g_is_f12_key_pressed
                && self.demo_manager.demo_file_name.len() != 0
            {
                self.demo_manager.g_is_sp_demo_available_to_run = 1;
                self.states.g_should_leave_main_menu = true;
                self.g_is_playing_demo = false;
                self.states.g_should_update_total_level_time = false;
                self.states.g_has_user_cheated = true;
                //prepareDemoRecordingFilename();
                // This adds dashes to the level name or something?
                self.demo_manager
                    .g_sp_demo_filename
                    .replace_range(3..6, "---");
                continue;
            }
            if self.mouse.button_status == MOUSE_BUTTON_RIGHT
            // Right button -> exit game
            {
                log::debug!("Right click");
                self.g_should_exit_game = true;
                break;
            } //else if self.keyboard.gIsEscapeKeyPressed
              // ||  getGameControllerButtonBack() TODO implement controller
              /*{
                  runAdvancedOptionsRootMenu();
              } else if (isRotateLevelSetAscendingButtonPressed()) {
                  self.throttled_rotate_level_set(0);
                  continue; // This allows the throttling effect to act
              } else if (isRotateLevelSetDescendingButtonPressed()) {
                  self.throttled_rotate_level_set(1);
                  continue; // This allows the throttling effect to act
              }*/
            if self.g_should_exit_game {
                break;
            }

            if self.mouse.button_status == MOUSE_BUTTON_LEFT {
                log::debug!("Left click : x = {}, y = {}", self.mouse.x, self.mouse.y);
                self.g_automatic_demo_playback_countdown = 4200;

                for i in 0..K_NUMBER_OF_MAIN_MENU_BUTTONS {
                    let button_descriptor = &K_MAIN_MENU_BUTTON_DESCRIPTORS[i];

                    //checkmousecoords:
                    if self.mouse.x >= button_descriptor.start_x
                        && self.mouse.y >= button_descriptor.start_y
                        && self.mouse.x <= button_descriptor.end_x
                        && self.mouse.y <= button_descriptor.end_y
                    {
                        log::debug!("Button find !");

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
                self.g_ranking_text_entries[self.current_ranking_index as usize + i as usize]
                    .clone(),
            );
        }

        // Display current ranking index in right box
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            144,
            110,
            6,
            format!("{:02}", self.current_ranking_index),
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

        for i in 0..K_NUMBER_OF_PLAYERS {
            if ranking_entries[i].player_index == self.states.g_current_player_index as u8 {
                self.byte_58D47 = i as u8;
            }
        }

        self.g_ranking_text_entries[0] = String::from(" ").repeat(22);
        self.g_ranking_text_entries[1] = String::from(" ").repeat(22);
        self.g_ranking_text_entries[K_NUMBER_OF_PLAYERS + 2] = String::from(" ").repeat(22);
        self.g_ranking_text_entries[K_NUMBER_OF_PLAYERS + 3] = String::from(" ").repeat(22);

        for i in 0..K_NUMBER_OF_PLAYERS {
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
            self.graphics.fade_to_palette(PaletteType::GamePalette);

            self.graphics.video_loop();

            // This will prevent to leave traces of the options menu
            // area in the main menu.
            //saveLastMouseAreaBitmap();
        } else {
            self.scroll_left_to_main_menu();
        }
    }

    fn draw_failed_level_result_screen(&mut self) {
        self.graphics.set_palette(PaletteType::Black);
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
        self.graphics
            .set_palette(PaletteType::InformationScreenPalette);

        if !self.g_should_exit_game {
            self.wait_for_key_press_or_mouse_click();
        }

        self.graphics.set_palette(PaletteType::Black);
    }

    fn scroll_left_to_main_menu(&mut self) {
        let current_screen_pixels = self.graphics.video.borrow_mut().get_screen_pixels();
        let menu_screen_pixels = self.graphics.video.borrow_mut().get_screen_pixels(); // Appel en double?

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

    fn scroll_right_to_new_screen(&mut self) {
        self.graphics.video_loop();

        let screen_pixel_backup = self.video.borrow_mut().get_screen_pixels();

        const NUMBER_OF_STEPS: u32 = 80;

        const ANIMATION_DURATION: u32 = NUMBER_OF_STEPS * 1000 / 70; // ~571 ms
        let mut animation_time = 0_u32;

        self.graphics.start_tracking_tender_delta_time();

        // Draws the current scroll animation step
        while animation_time < ANIMATION_DURATION {
            animation_time += self.graphics.update_render_delta_time();
            animation_time = std::cmp::min(animation_time, ANIMATION_DURATION);

            let animation_factor = animation_time as f64 / ANIMATION_DURATION as f64;

            let limit_from_right = animation_factor * K_SCREEN_WIDTH as f64;
            let limit_from_left = K_SCREEN_WIDTH as f64 - limit_from_right;
            let limit_from_right = limit_from_right as usize;
            let limit_from_left = limit_from_left as usize;

            for y in 0..K_SCREEN_HEIGHT {
                // Main menu side
                for x in 0..(K_SCREEN_WIDTH - limit_from_right) {
                    let color = screen_pixel_backup[y * K_SCREEN_WIDTH + x + limit_from_right];
                    self.graphics
                        .video
                        .borrow_mut()
                        .set_pixel(y * K_SCREEN_WIDTH + x, color);
                }

                // GFX background side
                for x in limit_from_left..K_SCREEN_WIDTH {
                    let color = self.graphics.get_pixel(
                        DestinationSurface::Scroll,
                        y * K_SCREEN_WIDTH + x - limit_from_left,
                    );
                    self.graphics
                        .video
                        .borrow_mut()
                        .set_pixel(y * K_SCREEN_WIDTH + x, color);
                }
            }

            self.graphics.video_loop();
        }
    }

    fn draw_main_menum_button_borders(&mut self) {
        let mut color = 0;

        if self.button_states.g_player_list_button_pressed != false {
            if self.button_states.g_player_list_up_button_pressed == false {
                color = 7;
            } else {
                color = 0xD; // 13
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[0], color);
            if self.button_states.g_player_list_up_button_pressed == false {
                color = 0xD; // 13
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[1], color);
            if self.button_states.g_player_list_down_button_pressed == false {
                color = 7;
            } else {
                color = 0xD; // 13
            }
            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[2], color);
            if self.button_states.g_player_list_down_button_pressed == false {
                color = 0xD; // 13
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[3], color);
            self.button_states.g_player_list_button_pressed = false;
        }

        if self.button_states.g_ranking_list_button_pressed != false {
            if self.button_states.g_ranking_list_up_button_pressed == false {
                color = 7;
            } else {
                color = 0xD; // 13
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[4], color);
            if self.button_states.g_ranking_list_up_button_pressed == false {
                color = 0xD;
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[5], color);
            if self.button_states.g_ranking_list_down_button_pressed == false {
                color = 7;
            } else {
                color = 0xD;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[6], color);
            if self.button_states.g_ranking_list_down_button_pressed == false {
                color = 0xD;
            } else {
                color = 7;
            }

            self.graphics
                .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[7], color);
            self.button_states.g_ranking_list_button_pressed = false;
        }

        if self.button_states.g_level_list_button_pressed == false {
            return;
        }
        if self.button_states.g_level_list_up_button_pressed == false {
            color = 7;
        } else {
            color = 0xD;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[8], color);
        if self.button_states.g_level_list_up_button_pressed == false {
            color = 0xD;
        } else {
            color = 7;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[9], color);
        if self.button_states.g_level_list_down_button_pressed == false {
            color = 7;
        } else {
            color = 0xD;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[10], color);
        if self.button_states.g_level_list_down_button_pressed == false {
            color = 0xD;
        } else {
            color = 7;
        }

        self.graphics
            .draw_main_menu_button_border(K_MAIN_MENU_BUTTON_BORDERS[11], color);
        self.button_states.g_level_list_button_pressed = false;
    }

    fn handle_new_player_option_click(&mut self) {
        log::info!("handle_new_player_option_click");

        if self.g_is_forced_cheat_mode {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                6,
                String::from("PLAYER LIST FULL       "),
            );
            return;
        }

        let mut new_player_index: i32 = -1;

        for i in 0..K_NUMBER_OF_PLAYERS {
            let current_player_entry = self.g_player_list_data[i].clone();

            if current_player_entry.name.eq("--------") {
                new_player_index = i as i32;
                break;
            }
        }

        if new_player_index == -1 {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                6,
                String::from("PLAYER LIST FULL       "),
            );
            return;
        }
        self.states.g_new_player_entry_index = new_player_index as usize;
        let mut new_player_name = String::from("        ");
        self.states.g_new_player_name_length = 0;
        //let  mouse_status = self.get_mouse_status();

        //restoreLastMouseAreaBitmap();

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            168,
            127,
            4,
            String::from("YOUR NAME:             "),
        );

        /*
        loop {
            let mouse_status = self.get_mouse_status();
            if mouse_status.button_status != 0 {
                break;
            }
        }
        */
        let mut last_pressed_character = '\0';

        loop {
            self.graphics.video_loop();

            //int9handler(0);
            let mouse_status = self.get_mouse_status();
            if mouse_status.button_status != 0 {
                break;
            }
            self.update_keyboard_state();
            {
                let is_any_key_pressed = self.keyboard.borrow_mut().is_any_key_pressed();
                if is_any_key_pressed == false {
                    last_pressed_character = '\0';
                    continue;
                }
            }
            let character = self.keyboard.borrow_mut().character_for_last_key_pressed();

            if last_pressed_character == character {
                continue;
            }

            last_pressed_character = character;

            if character == '\0'
            // For keys without a valid representation
            {
                continue;
            }
            if character == '\n'
            // \n -> enter -> create player
            {
                log::debug!("Creating player {new_player_name}");
                break;
            }
            if character == '%'
            // backspace -> delete last char
            {
                if self.states.g_new_player_name_length == 0 {
                    continue;
                }
                self.states.g_new_player_name_length -= 1;
                let range_start = self.states.g_new_player_name_length as usize;
                new_player_name.replace_range(range_start..(range_start + 1), " ");
                self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                    232,
                    127,
                    6,
                    new_player_name.clone(),
                );
                continue;
            }
            if self.states.g_new_player_name_length >= 8
            // when more than 8 chars were entered, ignore the rest?
            {
                continue;
            }
            let range_start = self.states.g_new_player_name_length as usize;
            new_player_name.replace_range(
                range_start..(range_start + 1),
                String::from(character).as_str(),
            );
            self.states.g_new_player_name_length += 1;
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                232,
                127,
                6,
                new_player_name.clone(),
            );
        }

        /*
        loop {
            let mouse_status = self.get_mouse_status();
            if mouse_status.button_status != 0 {
                break;
            }
        }
        */
        // Completely empty name: ignore
        if new_player_name == "        " {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                String::from("                       "),
            );
            //saveLastMouseAreaBitmap();
            //drawMouseCursor();
            return;
        }

        if new_player_name == "--------" {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                6,
                String::from("INVALID NAME           "),
            );
            //saveLastMouseAreaBitmap();
            //drawMouseCursor();
            return;
        }

        // Move spaces at the end of the name to the beginning
        let trimmed_player_name = new_player_name.trim();
        let space_quantity = K_PLAYER_NAME_LENGTH - trimmed_player_name.len();
        let new_player_name = String::from(" ").repeat(space_quantity) + trimmed_player_name;

        for i in 0..K_NUMBER_OF_PLAYERS {
            if self.g_player_list_data[i].name == new_player_name {
                self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                    168,
                    127,
                    6,
                    String::from("PLAYER EXISTS          "),
                );
                //saveLastMouseAreaBitmap();
                //drawMouseCursor();
                return;
            }
        }

        self.states.g_current_player_index = self.states.g_new_player_entry_index;
        self.g_player_list_data[self.states.g_current_player_index].name = new_player_name;

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            168,
            127,
            8,
            String::from("                       "),
        );
        self.save_player_list_data();
        self.save_hall_of_fame_data();
        self.g_should_autoselect_next_level_to_play = true;
        self.prepare_level_data_for_current_player();
        self.draw_player_list();
        self.draw_level_list();
        self.draw_rankings();
        //saveLastMouseAreaBitmap();
        //drawMouseCursor();
    }

    fn handle_delete_player_option_click(&mut self) {
        log::info!("handle_delete_player_option_click");
        if self.g_is_forced_cheat_mode {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                "NO PLAYER SELECTED     ".to_string(),
            );
            return;
        }

        let current_player_name = self.g_player_list_data[self.states.g_current_player_index]
            .name
            .clone();
        if current_player_name == "--------".to_string() {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                "NO PLAYER SELECTED     ".to_string(),
            );
            return;
        }

        let message = format!("DELETE '{}' ???  ", current_player_name);

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(168, 127, 8, message);

        let mut mouse_x = 0;
        let mut mouse_y = 0;

        loop {
            self.graphics.video_loop();
            let mouse_status = self.get_mouse_status();
            mouse_x = mouse_status.x;
            mouse_y = mouse_status.y;
            if mouse_status.button_status != 0 {
                break;
            }
        }
        let ok_button_descriptor = &K_MAIN_MENU_BUTTON_DESCRIPTORS[9];

        if mouse_x >= ok_button_descriptor.start_x
            && mouse_y >= ok_button_descriptor.start_y
            && mouse_x <= ok_button_descriptor.end_x
            && mouse_y <= ok_button_descriptor.end_y
        {
            self.g_player_list_data[self.states.g_current_player_index] = Box::new(PlayerEntry::new());
        }

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            168,
            127,
            8,
            "                       ".to_string(),
        );
        self.save_player_list_data();
        self.save_hall_of_fame_data();
        self.g_should_autoselect_next_level_to_play = true;
        self.prepare_level_data_for_current_player();
        self.draw_player_list();
        self.draw_level_list();
        self.draw_rankings();
    }

    fn handle_skip_level_option_click(&mut self) {
        log::info!("handle_skip_level_option_click");
        // 01ED:419C
        let current_player_entry =
            self.g_player_list_data[self.states.g_current_player_index].clone();

        if current_player_entry.name == "--------" {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                "NO PLAYER SELECTED     ".to_string(),
            );
            return;
        }

        let mut number_of_skipped_levels = 0;

        for i in 0..K_NUMBER_OF_LEVELS {
            if current_player_entry.level_state[i] == PlayerLevelState::Skipped {
                number_of_skipped_levels += 1;
            }
        }

        if self.g_is_debug_mode_enabled == false {
            if number_of_skipped_levels >= 3 {
                self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                    168,
                    127,
                    6,
                    "SKIP NOT POSSIBLE      ".to_string(),
                );
                return;
            }
        }

        if self
            .states
            .get_g_current_player_level_data(self.states.g_current_player_index)
            != K_NOT_COMPLETED_LEVEL_ENTRY_COLOR
        {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                4,
                "COLORBLIND I GUESS     ".to_string(),
            );
            return;
        }

        let message = format!(
            "SKIP LEVEL {:03} ???     ",
            self.states.g_current_selected_level_index
        );
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(168, 127, 8, message);

        loop {
            let mouse_status = self.get_mouse_status();
            if mouse_status.button_status != 0 {
                break;
            }
        }

        let mut mouse_x;
        let mut mouse_y;

        loop {
            self.graphics.video_loop();
            let mouse_status = self.get_mouse_status();
            mouse_x = mouse_status.x;
            mouse_y = mouse_status.y;
            if mouse_status.button_status != 0 {
                break;
            }
        }

        let ok_button_descriptor = &K_MAIN_MENU_BUTTON_DESCRIPTORS[9];

        if mouse_x >= ok_button_descriptor.start_x
            && mouse_y >= ok_button_descriptor.start_y
            && mouse_x <= ok_button_descriptor.end_x
            && mouse_y <= ok_button_descriptor.end_y
        {
            self.states.g_current_player_level_state = PlayerLevelState::Skipped;
            self.change_player_current_level_state();
            self.g_should_autoselect_next_level_to_play = false;
            self.prepare_level_data_for_current_player();
        }

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            168,
            127,
            8,
            "                       ".to_string(),
        );
        self.draw_player_list();
        self.draw_level_list();
        self.draw_rankings();

        loop {
            let mouse_status = self.get_mouse_status();
            if mouse_status.button_status != 0 {
                break;
            }
        }
    }

    fn handle_statistics_option_click(&mut self) {
        log::info!("handle_statistics_option_click");
        let current_player_entry =
            self.g_player_list_data[self.states.g_current_player_index].clone();
        if current_player_entry.name == "--------".to_string() {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                "NO PLAYER SELECTED     ".to_string(),
            );
            return;
        }

        self.graphics.fade_to_palette(PaletteType::Black);

        let screen_pixel_backup = self.video.borrow_mut().get_screen_pixels();

        self.graphics.draw_back_background();

        let mut special_text = 0;

        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            80,
            20,
            15,
            "SUPAPLEX  BY DREAM FACTORY".to_string(),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            64,
            50,
            15,
            "(C) DIGITAL INTEGRATION LTD 1991".to_string(),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            16,
            60,
            15,
            "________________________________________________".to_string(),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            80,
            80,
            15,
            "SUPAPLEX PLAYER STATISTICS".to_string(),
        );

        let current_player_text = format!("CURRENT PLAYER :  {}", current_player_entry.name);
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            80,
            100,
            15,
            current_player_text,
        );

        if current_player_entry.next_level_to_play == K_LAST_LEVEL_INDEX as u8 {
            special_text = 1;
        }

        let current_level_text = format!(
            "CURRENT LEVEL  :       {:03}",
            current_player_entry.next_level_to_play
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            80,
            110,
            15,
            current_level_text,
        );

        let used_time_text = format!(
            "USED TIME      : {:03}:{:02}:{:02}",
            current_player_entry.hours, current_player_entry.minutes, current_player_entry.seconds
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            80,
            120,
            15,
            used_time_text,
        );

        let mut total_minutes =
            current_player_entry.hours as u32 * 60 + current_player_entry.minutes as u32;

        if current_player_entry.seconds >= 30 {
            total_minutes += 1;
        }

        let average_time_string = "000.0";
        let average_minutes_whole = total_minutes / current_player_entry.next_level_to_play as u32;
        let average_minutes_fraction =
            total_minutes % current_player_entry.next_level_to_play as u32;
        let average_minutes_fraction =
            average_minutes_fraction / current_player_entry.next_level_to_play as u32;
        //convertNumberTo3DigitStringWithPadding0(average_minutes_fraction, &average_time_string[2]);

        if average_minutes_whole == 0 {
            special_text = 2;
        }

        let average_time_string =
            format!("{:03}.{}", average_minutes_whole, average_minutes_fraction);

        //convertNumberTo3DigitPaddedString(average_minutes_whole, average_time_string, 1);
        if special_text == 1 {
            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                24,
                140,
                15,
                "YOU'VE COMPLETED ALL LEVELS! CONGRATULATIONS!!!".to_string(),
            );
        } else if special_text == 2 {
            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                40,
                140,
                15,
                "STILL UNDER ONE MINUTE (KEEP IT UP...)".to_string(),
            );
        } else {
            let average_time_message = format!(
                "AVERAGE TIME USED PER LEVEL  {} MINUTES",
                average_time_string
            );
            self.draw_text_with_chars6_font_with_transparent_background_if_possible(
                32,
                140,
                15,
                average_time_message,
            );
        }

        self.graphics
            .fade_to_palette(PaletteType::InformationScreenPalette);

        self.wait_for_key_press_or_mouse_click();
        self.graphics.fade_to_palette(PaletteType::Black);

        self.video
            .borrow_mut()
            .set_screen_pixels(screen_pixel_backup);

        self.graphics.fade_to_palette(PaletteType::GamePalette);
    }
    fn handle_gfx_tutor_option_click(&mut self) {
        log::info!("handle_gfx_tutor_option_click");
        self.graphics
            .draw_gfx_tutor_background(DestinationSurface::Scroll);
        self.scroll_right_to_new_screen();
        self.wait_for_key_press_or_mouse_click();
        self.scroll_left_to_main_menu();
        self.draw_menu_title_and_demo_level_result();
    }
    fn handle_demo_option_click(&mut self) {
        log::info!("handle_demo_option_click");
        if self.demo_manager.read_demo_files() == 0 {
            return;
        }

        self.states.g_should_leave_main_menu = true;
        self.states.g_is_playing_demo = true;

        let mut number_of_demos = 0;

        let mut idx = 0; // usefull ? number_of_demos seems enough
        loop {
            if self.demo_manager.g_demos.demo_first_indices[idx] == 0xffff {
                break;
            }
            idx += 1;
            number_of_demos += 1;
        }

        // This picks a random demo
        self.generate_random_seed_from_clock();
        let demo_index = self.generate_random_number() % number_of_demos;
        let mut demo_first_index =
            self.demo_manager.g_demos.demo_first_indices[demo_index as usize];

        // This only happens if there are no demos...
        if demo_first_index == 0xffff {
            self.states.g_should_leave_main_menu = false;
            self.states.g_is_playing_demo = false;
        }

        let demo_level_number = self.demo_manager.g_demos.demo_data[demo_first_index as usize];
        let mut final_level_number = demo_index;

        self.demo_manager.g_selected_original_demo_index = demo_index;
        self.demo_manager.g_selected_original_demo_level_number = 0;

        // This checks if the level number has its MSB to 0 and is a valid level number (1-111) for the original DEMO format
        if demo_level_number <= 0x6F // 111
            && demo_level_number != 0
        {
            self.demo_manager.g_selected_original_demo_level_number =
                (self.demo_manager.g_selected_original_demo_level_number & 0xFF00)
                    | (demo_level_number as usize); // mov byte ptr gSelectedOriginalDemoLevelNumber, al
            final_level_number = demo_level_number as u16;
        }

        self.g_random_generator_seed = self.demo_manager.g_demo_random_seeds[demo_index as usize];
        self.demo_manager.g_selected_original_demo_level_number = final_level_number as usize;

        demo_first_index += 1; // To skip the level number
        self.demo_manager.g_demo_current_input_index = demo_first_index;
        self.demo_manager.word_5A33C = demo_first_index;
        self.demo_manager.g_demo_current_input = UserInput::None;
        self.demo_manager.g_demo_current_input_repeater_count = 1;
    }

    fn handle_controls_option_click(&mut self) {
        log::info!("handle_controls_option_click");
        self.byte_50919 = 0xFF;
        self.graphics
            .draw_options_background(DestinationSurface::Scroll);
        self.draw_sound_type_options_selection(DestinationSurface::Scroll);
        self.draw_audio_options_selection(DestinationSurface::Scroll);
        self.draw_input_options_selection(DestinationSurface::Scroll);

        self.graphics.set_palette(PaletteType::ControlsPalette);
        self.scroll_right_to_new_screen();
        self.should_quit_option_menu = false;

        loop {
            self.graphics.video_loop(); // 01ED:5E04
            self.update_options_menu_state(DestinationSurface::Screen);
            self.states.g_frame_counter += 1;
            let mouse_status = self.get_mouse_status();

            if mouse_status.button_status == MOUSE_BUTTON_RIGHT {
                break;
            }
            if self.input.is_menu_back_button_pressed()
            // Select/Back/- controller button -> go back
            {
                break;
            }
            if self.should_quit_option_menu == true {
                break;
            }
            if mouse_status.button_status == MOUSE_BUTTON_LEFT {
                for i in 0..K_NUMBER_OF_OPTIONS_MENU_BUTTONS {
                    let button_descriptor = &K_OPTIONS_MENU_BUTTON_DESCRIPTORS[i];
                    if mouse_status.x >= button_descriptor.start_x
                        && mouse_status.y >= button_descriptor.start_y
                        && mouse_status.x <= button_descriptor.end_x
                        && mouse_status.y <= button_descriptor.end_y
                    {
                        log::debug!("Button find !");
                        (button_descriptor.callback)(self);

                        loop {
                            self.graphics.video_loop();
                            self.states.g_frame_counter += 1;
                            let mouse_status = self.get_mouse_status();
                            if mouse_status.button_status != 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }

        self.save_configuration();
        self.scroll_left_to_main_menu();
        self.draw_menu_title_and_demo_level_result();
        self.graphics.set_palette(PaletteType::GamePalette);
    }

    fn handle_options_adlib_click(&mut self) {
        log::info!("handle_options_adlib_click");
        log::info!("handle_options_combined_click");
        self.sounds.activate_adlib_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_sound_blaster_click(&mut self) {
        log::info!("handle_options_sound_blaster_click");
        log::info!("handle_options_combined_click");
        self.sounds.activate_sound_blaster_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_roland_click(&mut self) {
        log::info!("handle_options_roland_click");
        log::info!("handle_options_combined_click");
        self.sounds.activate_roland_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_combined_click(&mut self) {
        log::info!("handle_options_combined_click");
        self.sounds.activate_combined_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_internal_click(&mut self) {
        log::info!("handle_options_internal_click");
        log::info!("handle_options_combined_click");
        self.sounds.activate_internal_standard_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_standard_click(&mut self) {
        log::info!("handle_options_standard_click");
        log::info!("handle_options_combined_click");
        self.sounds.activate_internal_standard_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_samples_click(&mut self) {
        log::info!("handle_options_samples_click");
        log::info!("handle_options_combined_click");
        self.sounds.activate_internal_samples_sound();
        self.sounds.play_explosion_sound();
        self.draw_sound_type_options_selection(DestinationSurface::Screen);
    }

    fn handle_options_music_click(&mut self) {
        log::info!("handle_options_music_click");
        if self.sounds.is_music_enabled {
            self.sounds.is_music_enabled = false;
            self.sounds.stop_music();
        } else {
            self.sounds.is_music_enabled = true;
            self.sounds.play_music_if_needed();
        }
        self.draw_audio_options_selection(DestinationSurface::Screen);
    }

    fn handle_options_fx_click(&mut self) {
        log::info!("handle_options_fx_click");
        if self.sounds.is_fx_enabled {
            self.sounds.is_fx_enabled = false;
        } else {
            self.sounds.is_fx_enabled = true;
            self.sounds.play_explosion_sound();
        }
        self.draw_audio_options_selection(DestinationSurface::Screen);
    }

    fn handle_options_keyboard_click(&mut self) {
        log::info!("handle_options_keyboard_click");
        self.is_joystick_enabled = false;
        self.draw_input_options_selection(DestinationSurface::Screen);
    }

    fn handle_options_joystick_click(&mut self) {
        log::info!("handle_options_joystick_click");
        self.is_joystick_enabled = true;
        self.draw_input_options_selection(DestinationSurface::Screen);
    }
    fn handle_options_exit_area_click(&mut self) {
        log::info!("handle_options_exit_area_click");
        self.should_quit_option_menu = true;
    }

    fn handle_ranking_list_scroll_up(&mut self) {
        log::info!("handle_ranking_list_scroll_up");
        self.button_states.g_ranking_list_button_pressed = true;
        self.button_states.g_ranking_list_down_button_pressed = false;
        self.button_states.g_ranking_list_up_button_pressed = true;

        if self.states.g_frame_counter - self.g_ranking_list_throttle_current_counter
            < self.g_ranking_list_throttle_next_counter
        {
            return;
        }

        self.g_ranking_list_throttle_next_counter = self.states.g_frame_counter;
        if self.g_ranking_list_throttle_current_counter > 1 {
            self.g_ranking_list_throttle_current_counter -= 1;
        }

        if self.g_is_forced_cheat_mode == false && self.current_ranking_index > 0 {
            self.current_ranking_index -= 1;
        }

        self.draw_rankings();
    }

    fn handle_ranking_list_scroll_down(&mut self) {
        log::info!("handle_ranking_list_scroll_down");
        self.button_states.g_ranking_list_button_pressed = true;
        self.button_states.g_ranking_list_down_button_pressed = true;
        self.button_states.g_ranking_list_up_button_pressed = false;

        if self.states.g_frame_counter - self.g_ranking_list_throttle_current_counter
            < self.g_ranking_list_throttle_next_counter
        {
            return;
        }

        self.g_ranking_list_throttle_next_counter = self.states.g_frame_counter;
        if self.g_ranking_list_throttle_current_counter > 1 {
            self.g_ranking_list_throttle_current_counter -= 1;
        }

        if self.g_is_forced_cheat_mode == false
            && self.current_ranking_index < K_NUMBER_OF_PLAYERS as u8 - 1
        {
            self.current_ranking_index += 1;
        }

        self.draw_rankings();
    }

    fn handle_ok_button_click(&mut self) {
        log::info!("handle_ok_button_click");

        if self.g_player_list_data[self.states.g_current_player_index].name
            == "--------".to_string()
        {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                "NO PLAYER SELECTED     ".to_string(),
            );
            return;
        }

        if self.states.g_current_selected_level_index == K_LAST_LEVEL_INDEX as u8 {
            log::debug!("self.states.g_current_selected_level_index == K_LAST_LEVEL_INDEX");
            let mut number_of_completed_levels = 0;

            for i in 0..K_NUMBER_OF_LEVELS {
                if self.g_player_list_data[self.states.g_current_player_index].level_state[i]
                    == PlayerLevelState::Completed
                {
                    number_of_completed_levels += 1;
                }
            }
            if number_of_completed_levels == K_NUMBER_OF_LEVELS {
                self.show_congratulations_screen();
                return;
            } else {
                log::debug!("oups");
                self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                    168,
                    127,
                    2,
                    "COLORBLIND I GUESS     ".to_string(),
                );
                return;
            }
        } else if self.states.g_current_selected_level_index > K_NUMBER_OF_LEVELS as u8 {
            return;
        }

        let current_level_color = self.states.g_current_player_padded_level_data
            [K_FIRST_LEVEL_INDEX + self.states.g_current_selected_level_index as usize - 1];

        log::debug!("Index : {}", self.states.g_current_selected_level_index);
        log::debug!("Color : {}", current_level_color);

        if current_level_color == K_BLOCKED_LEVEL_ENTRY_COLOR {
            self.draw_text_with_chars6_font_with_opaque_background_if_possible(
                168,
                127,
                8,
                "COLORBLIND I GUESS     ".to_string(),
            );
            return;
        }
        self.states.g_should_leave_main_menu = true;
        self.g_is_playing_demo = false;
        if current_level_color == K_COMPLETED_LEVEL_ENTRY_COLOR {
            self.states.g_should_update_total_level_time = false;
        } else {
            self.states.g_should_update_total_level_time = true;
        }

        self.prepare_demo_recording_filename();
        self.convert_level_number_to_3_digit_string_with_padding_0(
            self.states.g_current_selected_level_index,
        )
    }

    fn handle_floppy_disk_button_click(&mut self) {
        log::info!("handle_floppy_disk_button_click");
        self.throttled_rotate_level_set(
            self.keyboard.borrow().g_is_right_shift_pressed
                || self.keyboard.borrow().g_is_left_shift_pressed,
        );
    }

    fn handle_player_list_scroll_up(&mut self) {
        log::info!("handle_player_list_scroll_up");
        self.button_states.g_player_list_button_pressed = true;
        self.button_states.g_player_list_down_button_pressed = false;
        self.button_states.g_player_list_up_button_pressed = true;

        if self.states.g_frame_counter - self.g_player_list_throttle_current_counter
            < self.g_player_list_throttle_next_counter
        {
            return;
        }

        self.g_player_list_throttle_next_counter = self.states.g_frame_counter;
        if self.g_player_list_throttle_current_counter > 1 {
            self.g_player_list_throttle_current_counter -= 1;
        }

        if self.g_is_forced_cheat_mode == false && self.states.g_current_player_index > 0 {
            self.states.g_current_player_index -= 1;
        }

        self.g_should_autoselect_next_level_to_play = true;
        self.prepare_level_data_for_current_player();
        self.draw_player_list();
        self.draw_level_list();
    }

    fn handle_player_list_scroll_down(&mut self) {
        log::info!("handle_player_list_scroll_down");
        // Pressing the shift key will show the level sets in descending order
        self.button_states.g_player_list_button_pressed = true;
        self.button_states.g_player_list_down_button_pressed = true;
        self.button_states.g_player_list_up_button_pressed = false;

        if self.states.g_frame_counter - self.g_player_list_throttle_current_counter
            < self.g_player_list_throttle_next_counter
        {
            return;
        }

        self.g_player_list_throttle_next_counter = self.states.g_frame_counter;
        if self.g_player_list_throttle_current_counter > 1 {
            self.g_player_list_throttle_current_counter -= 1;
        }

        if self.g_is_forced_cheat_mode == false
            && self.states.g_current_player_index < K_NUMBER_OF_PLAYERS - 1
        {
            self.states.g_current_player_index += 1;
        }

        self.g_should_autoselect_next_level_to_play = true;
        self.prepare_level_data_for_current_player();
        self.draw_player_list();
        self.draw_level_list();
    }

    fn handle_player_list_click(&mut self) {
        log::info!("handle_player_list_click");
        self.current_ranking_index = self.byte_58D47;
        self.draw_rankings();
    }

    fn handle_level_list_scroll_up(&mut self) {
        log::info!("handle_level_list_scroll_up");

        self.button_states.g_level_list_button_pressed = true;
        self.button_states.g_level_list_down_button_pressed = false;
        self.button_states.g_level_list_up_button_pressed = true;

        if self.states.g_frame_counter - self.g_level_list_throttle_current_counter
            < self.g_level_list_throttle_next_counter
        {
            return;
        }

        self.g_level_list_throttle_next_counter = self.states.g_frame_counter;
        if self.g_level_list_throttle_current_counter > 1 {
            self.g_level_list_throttle_current_counter -= 1;
        }

        if self.states.g_current_selected_level_index <= 1 {
            return;
        }
        self.states.g_current_selected_level_index -= 1;
        //restoreLastMouseAreaBitmap();
        self.draw_level_list();
        //saveLastMouseAreaBitmap();
        //drawMouseCursor();
    }

    fn handle_level_list_scroll_down(&mut self) {
        log::info!("handle_level_list_scroll_down");
        self.button_states.g_level_list_button_pressed = true;
        self.button_states.g_level_list_down_button_pressed = true;
        self.button_states.g_level_list_up_button_pressed = false;

        if self.states.g_frame_counter - self.g_level_list_throttle_current_counter
            < self.g_level_list_throttle_next_counter
        {
            return;
        }

        self.g_level_list_throttle_next_counter = self.states.g_frame_counter;
        if self.g_level_list_throttle_current_counter > 1 {
            self.g_level_list_throttle_current_counter -= 1;
        }

        if self.states.g_current_selected_level_index >= 113 {
            return;
        }
        self.states.g_current_selected_level_index += 1;
        //restoreLastMouseAreaBitmap();
        self.draw_level_list();
        //saveLastMouseAreaBitmap();
        //drawMouseCursor();
    }

    fn handle_level_credits_click(&mut self) {
        log::info!("handle_level_credits_click");

        self.graphics.fade_to_palette(PaletteType::Black);

        let screen_pixel_backup = self.video.borrow_mut().get_screen_pixels();

        self.graphics.draw_back_background();

        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            80,
            10,
            15,
            String::from("SUPAPLEX  BY DREAM FACTORY"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            56,
            40,
            15,
            String::from("ORIGINAL DESIGN BY PHILIP JESPERSEN"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            88,
            50,
            15,
            String::from("AND MICHAEL STOPP"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            56,
            90,
            15,
            String::from("NEARLY ALL LEVELS BY MICHEAL STOPP"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            64,
            100,
            15,
            String::from("A FEW LEVELS BY PHILIP JESPERSEN"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            56,
            110,
            15,
            String::from("HARDLY ANY LEVELS BY BARBARA STOPP"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            64,
            170,
            15,
            String::from("NOTE: PRESS ENTER TO REMOVE PANEL"),
        );
        self.draw_text_with_chars6_font_with_transparent_background_if_possible(
            64,
            190,
            15,
            String::from("(C) DIGITAL INTEGRATION LTD 1991"),
        );

        self.graphics
            .fade_to_palette(PaletteType::InformationScreenPalette);

        self.wait_for_key_press_or_mouse_click();

        self.graphics.fade_to_palette(PaletteType::Black);

        self.video
            .borrow_mut()
            .set_screen_pixels(screen_pixel_backup);

        self.graphics.fade_to_palette(PaletteType::GamePalette);
    }

    fn fun_level(&mut self) {
        // TODO : implement function
    }

    fn update_keyboard_state(&mut self) {
        let kb_state = self.events.keyboard_state();
        let keys = kb_state.pressed_scancodes();
        self.keyboard.borrow_mut().update_keyboard_state(keys);
    }

    fn update_user_input(&mut self) {
        let mut direction_key_was_pressed = 0;

        self.keyboard.borrow_mut().g_current_user_input = UserInput::None;

        if self.is_up_button_pressed() {
            self.keyboard.borrow_mut().g_current_user_input = UserInput::Up;
            direction_key_was_pressed = 1;
        }

        if self.is_left_button_pressed() {
            self.keyboard.borrow_mut().g_current_user_input = UserInput::Left;
            direction_key_was_pressed = 1;
        }

        if self.is_down_button_pressed() {
            self.keyboard.borrow_mut().g_current_user_input = UserInput::Down;
            direction_key_was_pressed = 1;
        }

        if self.is_right_button_pressed() {
            self.keyboard.borrow_mut().g_current_user_input = UserInput::Right;
            direction_key_was_pressed = 1;
        }

        if self.is_action_button_pressed() {
            if direction_key_was_pressed == 1 {
                self.keyboard.borrow_mut().g_current_user_input +=
                    K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET;
            } else {
                self.keyboard.borrow_mut().g_current_user_input = UserInput::SpaceOnly;
            }
        }
    }

    fn is_action_button_pressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Space)
    }

    fn is_up_button_pressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Up)
    }

    fn is_down_button_pressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Down)
    }

    fn is_left_button_pressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Left)
    }

    fn is_right_button_pressed(&mut self) -> bool {
        self.events
            .keyboard_state()
            .is_scancode_pressed(Scancode::Right)
    }

    fn get_mouse_status(&mut self) -> Mouse {
        // Returns coordinate X in CX (0-320) and coordinate Y in DX (0-200).
        // Also button status in BX.

        self.handle_system_events();

        self.events.pump_events();

        let mouse_state = self.events.mouse_state();

        let mut x = mouse_state.x();
        let mut y = mouse_state.y();
        let left_button_pressed = mouse_state.left() as u8;
        let right_button_pressed = mouse_state.right() as u8;

        let (window_width, window_height) = self.video.borrow_mut().get_window_size();

        if window_width != 0 && window_height != 0 {
            x = x * K_SCREEN_WIDTH as i32 / window_width as i32;
            y = y * K_SCREEN_HEIGHT as i32 / window_height as i32;
        }

        // Limit coordinates as in the original game
        x = utils::clamp(x, 16, 304);
        y = utils::clamp(y, 8, 192);
        Mouse {
            x,
            y,
            button_status: right_button_pressed << 1 | left_button_pressed,
        }
    }

    fn save_player_list_data(&mut self) {
        if self.g_is_forced_cheat_mode {
            return;
        }

        let path = format!("{}/{}", RESSOURCES_PATH, G_PLAYERS_LST_FILENAME);
        let player_lst_file_path = Path::new(&path);
        let mut file = File::create(player_lst_file_path)
            .expect(format!("Error while opening {}", G_PLAYERS_LST_FILENAME).as_str());

        for i in 0..K_NUMBER_OF_PLAYERS {
            file.write_all(&self.g_player_list_data[i].to_raw())
                .expect(format!("Error while writing to {}", G_PLAYERS_LST_FILENAME).as_str());
        }
    }

    fn save_hall_of_fame_data(&mut self) {
        if self.g_is_forced_cheat_mode {
            return;
        }

        let path = format!("{}/{}", RESSOURCES_PATH, G_HALL_OF_FAME_LST_FILENAME);
        let hof_lst_file_path = Path::new(&path);

        let mut file = File::create(hof_lst_file_path)
            .expect(format!("Error while opening {}", G_HALL_OF_FAME_LST_FILENAME).as_str());

        for i in 0..K_NUMBER_OF_HALL_OF_FAME_ENTRIES {
            file.write_all(&self.g_hall_of_fame_data[i].to_raw())
                .expect(format!("Error while writing to {}", G_HALL_OF_FAME_LST_FILENAME).as_str());
        }
    }

    fn change_player_current_level_state(&mut self) {
        if self.g_is_playing_demo {
            return;
        }
        if self.g_has_user_cheated {
            return;
        }
        let previous_state = self.states.g_current_player_level_state;
        self.states.g_current_player_level_state = PlayerLevelState::NotCompleted;

        self.g_player_list_data[self.states.g_current_player_index].level_state
            [self.states.g_current_selected_level_index as usize] = previous_state;

        self.states.g_current_selected_level_index += 1;
        self.update_hall_of_fame_entries(); // 01ED:6618

        // Added by me to prevent losing progress when switching levelsets after finishing a level
        self.save_player_list_data();
        self.save_hall_of_fame_data();
    }

    fn update_hall_of_fame_entries(&mut self) {
        if self.g_is_playing_demo {
            return;
        }

        let current_player_entry =
            self.g_player_list_data[self.states.g_current_player_index].clone();
        if current_player_entry.completed_all_levels != 0 {
            return;
        }

        let mut number_of_completed_levels = 0;

        for i in 0..K_NUMBER_OF_LEVELS {
            if current_player_entry.level_state[i] == PlayerLevelState::Completed {
                number_of_completed_levels += 1;
            }
        }

        if number_of_completed_levels != K_NUMBER_OF_LEVELS {
            return;
        }

        self.g_player_list_data[self.states.g_current_player_index].completed_all_levels = 1;

        let mut new_entry_insert_index = -1_i32;
        for i in 0..K_NUMBER_OF_HALL_OF_FAME_ENTRIES as i32 {
            let entry = self.g_hall_of_fame_data[i as usize].clone();

            if entry.hours == 0 && entry.minutes == 0 && entry.seconds == 0 {
                new_entry_insert_index = i;
                break;
            }

            if current_player_entry.hours < entry.hours {
                new_entry_insert_index = i;
                break;
            } else if current_player_entry.hours == entry.hours {
                if current_player_entry.minutes < entry.minutes {
                    new_entry_insert_index = i;
                    break;
                } else if current_player_entry.minutes == entry.minutes {
                    if current_player_entry.seconds < entry.seconds {
                        new_entry_insert_index = i;
                        break;
                    }
                }
            }
        }

        if new_entry_insert_index != -1 {
            // Shift the list to the right to make room for the new entry
            self.g_hall_of_fame_data[2] = self.g_hall_of_fame_data[1].clone();
            self.g_hall_of_fame_data[1] = self.g_hall_of_fame_data[0].clone();

            // Copy the player info into the new entry
            let mut new_entry = HallOfFameEntry::new();
            new_entry.hours = current_player_entry.hours;
            new_entry.minutes = current_player_entry.minutes;
            new_entry.seconds = current_player_entry.seconds;

            self.g_hall_of_fame_data[new_entry_insert_index as usize] = Box::new(new_entry);
        }
    }

    fn draw_sound_type_options_selection(&mut self, dest_buffer: DestinationSurface) {
        self.dim_options_button_text(40, 21, 40, 8, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[0], 4, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[1], 4, dest_buffer);

        self.dim_options_button_text(24, 57, 72, 8, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[2], 4, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[3], 4, dest_buffer);

        self.dim_options_button_text(32, 93, 56, 8, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[4], 4, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[5], 4, dest_buffer);

        self.dim_options_button_text(24, 129, 64, 8, dest_buffer);
        self.dim_options_button_text(136, 18, 72, 8, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[7], 4, dest_buffer);

        self.dim_options_button_text(128, 46, 40, 5, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[8], 4, dest_buffer);

        self.dim_options_button_text(176, 46, 40, 5, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[9], 4, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[6], 4, dest_buffer);

        if self.sounds.snd_type == SoundType::Adlib {
            self.highlight_options_button_text(40, 21, 40, 8, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[0], 6, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[1], 6, dest_buffer);
            return;
        } else if self.sounds.snd_type == SoundType::SoundBlaster {
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[3], 6, dest_buffer);

            if self.sounds.mus_type == SoundType::Adlib {
                self.highlight_options_button_text(24, 57, 72, 8, dest_buffer);
                self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[2], 6, dest_buffer);
                return;
            }

            self.highlight_options_button_text(24, 129, 64, 8, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[4], 6, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[6], 6, dest_buffer);
            return;
        } else if self.sounds.snd_type == SoundType::Roland {
            self.highlight_options_button_text(32, 93, 56, 8, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[4], 6, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[5], 6, dest_buffer);
            return;
        }

        self.highlight_options_button_text(136, 18, 72, 8, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[7], 6, dest_buffer);

        if self.sounds.snd_type == SoundType::InternalStandard {
            self.highlight_options_button_text(128, 46, 40, 5, dest_buffer); // Standard
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[8], 6, dest_buffer);
            return;
        }

        self.highlight_options_button_text(176, 46, 40, 5, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[9], 6, dest_buffer);
    }

    fn draw_audio_options_selection(&mut self, dest_buffer: DestinationSurface) {
        if self.sounds.is_music_enabled {
            self.highlight_options_button_text(134, 99, 40, 8, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[10], 6, dest_buffer);
        } else {
            self.dim_options_button_text(134, 99, 40, 8, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[10], 4, dest_buffer);
        }

        if self.sounds.is_fx_enabled {
            self.highlight_options_button_text(136, 138, 24, 8, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[11], 6, dest_buffer);
            return;
        }

        self.dim_options_button_text(136, 138, 24, 8, dest_buffer);
        self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[11], 4, dest_buffer);
    }

    fn draw_input_options_selection(&mut self, dest_buffer: DestinationSurface)
    // sub_4CCDF   proc near       ; CODE XREF: code:5B5Dp code:5B69p
    {
        if self.is_joystick_enabled == false {
            self.highlight_options_button_text(208, 87, 8, 62, dest_buffer);
            self.dim_options_button_text(240, 88, 8, 58, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[18], 4, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[19], 6, dest_buffer);
        } else {
            self.dim_options_button_text(208, 87, 8, 62, dest_buffer);
            self.highlight_options_button_text(240, 88, 8, 58, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[18], 6, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[19], 4, dest_buffer);
        }

        self.update_options_menu_state(dest_buffer);
    }

    fn highlight_options_button_text(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        dest_buffer: DestinationSurface,
    ) {
        // Copies a portion of the buffer replacing color 0xF (not selected)
        // with color 0x1 (selected).
        // Used in the options screen to highlight text from buttons.
        //
        // Parameters:
        // - si: origin coordinates
        // - cx: width / 8
        // - dx: height

        for y in start_y..(start_y + height) {
            for x in start_x..(start_x + width) {
                let addr = (y * K_SCREEN_WIDTH) + x;

                let color = if self.graphics.get_pixel(dest_buffer, addr) == 0xF {
                    0x1
                } else {
                    self.graphics.get_pixel(dest_buffer, addr)
                };
                self.graphics.set_pixel(dest_buffer, addr, color);
            }
        }
    }

    fn dim_options_button_text(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
        dest_buffer: DestinationSurface,
    ) {
        // Copies a portion of the buffer replacing color 0x1 (selected)
        // with color 0xF (not selected).
        // Used in the options screen to dim text from buttons.
        //
        // Parameters:
        // - si: coordinates
        // - cx: width / 8
        // - dx: height

        for y in start_y..(start_y + height) {
            for x in start_x..(start_x + width) {
                let addr = (y * K_SCREEN_WIDTH) + x;

                let color = if self.graphics.get_pixel(dest_buffer, addr) == 0x1 {
                    0xF
                } else {
                    self.graphics.get_pixel(dest_buffer, addr)
                };
                self.graphics.set_pixel(dest_buffer, addr, color);
            }
        }
    }

    fn draw_options_menu_line(
        &mut self,
        border: &[ButtonBorderLineDescriptor],
        color: u8,
        dest_buffer: DestinationSurface,
    ) {
        // Parameters:
        // - ah: color
        // - si: pointer to ButtonBorderDescriptor item

        for i in 0..border.len() {
            let line = &border[i];

            let x = line.x as usize;
            let y = line.y as usize;

            for j in 0..line.length {
                let offset = j as usize;
                let mut dest_address = 0;
                if line.button_type == ButtonBorderLineType::Horizontal {
                    dest_address = y * K_SCREEN_WIDTH + x + offset;
                } else if line.button_type == ButtonBorderLineType::Vertical {
                    dest_address = (y - offset) * K_SCREEN_WIDTH + x;
                } else if line.button_type == ButtonBorderLineType::LeftToTopRightDiagonal {
                    dest_address = (y - offset) * K_SCREEN_WIDTH + x + offset;
                } else if line.button_type == ButtonBorderLineType::TopLeftToBottomRightDiagonal {
                    dest_address = (y + offset) * K_SCREEN_WIDTH + x + offset;
                }

                self.graphics.set_pixel(dest_buffer, dest_address, color);
            }
        }
    }

    fn update_options_menu_state(&mut self, dest_buffer: DestinationSurface) {
        self.update_user_input();

        let current_input = self.keyboard.borrow_mut().g_current_user_input;

        if current_input as u8 == self.byte_50919 {
            return;
        }

        self.byte_50919 = current_input as u8;
        if current_input == UserInput::None {
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[12], 6, dest_buffer);
            self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[17], 4, dest_buffer);
        } else {
            if current_input <= K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET {
                self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[12], 4, dest_buffer);
                self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[17], 4, dest_buffer);
            } else {
                self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[17], 6, dest_buffer);
                if current_input != UserInput::SpaceOnly {
                    self.keyboard.borrow_mut().g_current_user_input =
                        match self.keyboard.borrow_mut().g_current_user_input {
                            UserInput::SpaceDown => UserInput::Down,
                            UserInput::SpaceUp => UserInput::Up,
                            UserInput::SpaceLeft => UserInput::Left,
                            UserInput::SpaceRight => UserInput::Right,
                            _ => self.keyboard.borrow_mut().g_current_user_input,
                        };
                    self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[12], 4, dest_buffer);
                } else {
                    self.keyboard.borrow_mut().g_current_user_input = UserInput::None;
                    self.draw_options_menu_line(K_OPTIONS_MENU_BORDERS[12], 6, dest_buffer);
                }
            }
        }
    }

    fn save_configuration(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_CONFIG_FILE_NAME);
        let player_lst_file_path = Path::new(&path);
        let mut file = File::create(player_lst_file_path)
            .expect(format!("Error while opening {}", G_CONFIG_FILE_NAME).as_str());

        let mut config_data: [u8; K_CONFIG_DATA_LENGTH] = [0; K_CONFIG_DATA_LENGTH];

        if self.sounds.snd_type == SoundType::InternalSamples {
            config_data[0] = 's' as u8;
        } else if self.sounds.snd_type == SoundType::InternalStandard {
            config_data[0] = 'i' as u8;
        } else if self.sounds.snd_type == SoundType::Adlib {
            config_data[0] = 'a' as u8;
        } else if self.sounds.snd_type == SoundType::Roland {
            config_data[0] = 'r' as u8;
        } else if self.sounds.mus_type == SoundType::Roland {
            config_data[0] = 'c' as u8;
        } else {
            config_data[0] = 'b' as u8;
        }
        if self.is_joystick_enabled == false {
            config_data[1] = 'k' as u8;
        } else {
            config_data[1] = 'j' as u8;
        }

        if self.sounds.is_music_enabled {
            config_data[2] = 'm' as u8;
        } else {
            config_data[2] = 'n' as u8;
        }

        if self.sounds.is_fx_enabled {
            config_data[3] = 'x' as u8;
        } else {
            config_data[3] = 'y' as u8;
        }

        file.write_all(&config_data)
            .expect(format!("Error writing {}", G_CONFIG_FILE_NAME).as_str());
    }

    fn throttled_rotate_level_set(&mut self, descending: bool) {
        if self.states.g_frame_counter - self.g_level_set_rotation_throttle_current_counter
            < self.g_level_set_rotation_throttle_next_counter
        {
            return;
        }

        self.g_level_set_rotation_throttle_next_counter = self.states.g_frame_counter;
        if self.g_level_set_rotation_throttle_current_counter > 1 {
            self.g_level_set_rotation_throttle_current_counter -= 1;
        }

        self.rotate_level_set(descending);
    }

    fn rotate_level_set(&mut self, descending: bool) {
        let mut current_suffix = "AT";
        let mut new_suffix;
        loop {
            current_suffix = self.level_manager.g_levels_dat_filename.get(8..).unwrap();

            if descending {
                if current_suffix == "AT"
                // "AT"
                {
                    new_suffix = "99".to_string();
                } else if current_suffix == "00"
                // "00"
                {
                    new_suffix = "AT".to_string();
                } else {
                    let mut index = current_suffix.parse::<u8>().unwrap();
                    index -= 1;
                    new_suffix = format!("{:02}", index);
                }
            } else {
                if current_suffix == "AT"
                // "AT"
                {
                    new_suffix = "00".to_string();
                } else if current_suffix == "99"
                // "99"
                {
                    new_suffix = "AT".to_string();
                } else {
                    let mut index = current_suffix.parse::<u8>().unwrap();
                    index += 1;
                    new_suffix = format!("{:02}", index);
                }
            }

            self.level_manager.g_levels_dat_filename = format!(
                "{}{}",
                self.level_manager.g_levels_dat_filename.get(0..8).unwrap(),
                new_suffix
            );
            self.demo_manager.g_levels_dat_filename =
                self.level_manager.g_levels_dat_filename.clone();

            let path = format!(
                "{}/{}",
                RESSOURCES_PATH, self.level_manager.g_levels_dat_filename
            );

            if Path::new(path.as_str()).exists() {
                break;
            }
        }
        log::debug!(
            "New level file loaded: {}",
            self.level_manager.g_levels_dat_filename
        );

        self.files.change_suffix(new_suffix.as_str());

        self.level_manager.read_levels_lst();
        self.demo_manager.read_demo_files();

        if self.g_is_forced_cheat_mode {
            self.g_player_list_data[0].level_state = [PlayerLevelState::Skipped; K_NUMBER_OF_LEVELS]
        } else {
            self.g_player_list_data = Box::new([(); K_NUMBER_OF_PLAYERS].map(|_| Box::new(PlayerEntry::new())));
            self.g_hall_of_fame_data =
            Box::new([(); K_NUMBER_OF_HALL_OF_FAME_ENTRIES].map(|_| Box::new(HallOfFameEntry::new())));
            self.read_hall_fame_lst();
            self.read_players_lst();
        }

        self.update_menu_after_level_set_changed();
    }

    fn update_menu_after_level_set_changed(&mut self) {
        let current_suffix = self.level_manager.g_levels_dat_filename.get(8..).unwrap();

        let message = if current_suffix == "AT" {
            "  SUPAPLEX LEVEL SET   ".to_string()
        } else {
            format!("     LEVEL SET {}      ", current_suffix)
        };

        self.draw_text_with_chars6_font_with_opaque_background_if_possible(168, 127, 4, message);

        self.g_should_autoselect_next_level_to_play = true;
        self.prepare_level_data_for_current_player();
        self.draw_player_list();
        self.draw_level_list();
        self.draw_hall_of_fame();
        self.draw_rankings();
    }

    fn show_congratulations_screen(&mut self) {
        self.graphics.fade_to_palette(PaletteType::Black);

        let screen_pixel_backup = self.video.borrow_mut().get_screen_pixels();

        self.graphics.draw_back_background();
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            120,
            30,
            15,
            "CONGRATULATIONS".to_string(),
        );
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            24,
            70,
            15,
            "YOU HAVE COMPLETED ALL 111 LEVELS OF SUPAPLEX".to_string(),
        );
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            64,
            85,
            15,
            "YOUR BRAIN IS IN FANTASTIC SHAPE".to_string(),
        );
        self.draw_text_with_chars6_font_with_opaque_background_if_possible(
            40,
            100,
            15,
            "NOT MANY PEOPLE ARE ABLE TO MANAGE THIS".to_string(),
        );
        self.graphics
            .fade_to_palette(PaletteType::InformationScreenPalette);
        self.wait_for_key_press_or_mouse_click();
        self.graphics.fade_to_palette(PaletteType::Black);
        self.video
            .borrow_mut()
            .set_screen_pixels(screen_pixel_backup);

        self.graphics.fade_to_palette(PaletteType::GamePalette);
    }

    fn prepare_demo_recording_filename(&mut self) {
        let mut current_suffix = self.level_manager.g_levels_dat_filename.get(8..).unwrap();

        // Checks if the last two chars are "00" like LEVELS.D00?
        if current_suffix == "00" {
            // replaces the content with "--"
            current_suffix = "--";
        }
        // Now checks if the last two chars are "AT" like LEVELS.DAT?
        else if current_suffix == "AT" {
            // replaces the content with "00"
            current_suffix = "00";
        }

        log::debug!(
            "prepare_demo_recording_filename. filename={}",
            self.demo_manager.g_sp_demo_filename
        );

        self.demo_manager
            .g_sp_demo_filename
            .replace_range(9..11, current_suffix);
    }

    fn initialize_game_info(&mut self) {
        self.states.g_is_murphy_looking_left = false;
        self.states.g_should_kill_murphy = false;
        self.states.g_should_exit_level = false;
        self.states.g_quit_level_countdown = 0;
        self.states.g_number_of_remaining_red_disks = 0;
        self.states.g_additional_info_in_game_panel_frame_counter = 0;
        self.states.g_murphy_yawn_and_sleep_counter = 0;
        self.states.g_last_drawn_minutes_and_seconds = 0xffff;
        self.states.g_last_drawn_hours = 0xff; // 255
        self.states.g_is_game_running = true;
        self.states.g_aux_game_seconds_20ms_accumulator = 0;
        self.states.g_game_seconds = 0;
        self.states.g_game_minutes = 0;
        self.states.g_game_hours = 0;
        self.states.g_is_explosion_started = false;
        self.states.g_terminal_max_frames_to_next_scroll = 0x7F; // 127
        self.states.g_are_yellow_disks_detonated = false;
        self.states.g_frame_counter = 0;
        //    mov byte ptr word_510C1, 1
        //    mov byte ptr word_510C1+1, 0
        self.states.g_should_show_game_panel = true;
        self.states.g_current_panel_height = K_PANEL_BITMAP_HEIGHT;
        self.states.g_are_enemies_frozen = false;
        self.states.g_is_murphy_going_through_portal &= 0xff00; // mov byte ptr gIsMurphyGoingThroughPortal, 0
        self.states.g_planted_red_disk_countdown = 0;
        self.states.g_planted_red_disk_position = 0;
    }

    fn draw_fixed_level(&mut self) {
        if self.graphics.fast_mode == FastModeType::Ultra {
            return;
        }

        const K_MOVING_BITMAP_TOP_LEFT_CORNER_X: u16 = 288;
        const K_MOVING_BITMAP_TOP_LEFT_CORNER_Y: u16 = 388;
        const K_MOVING_BITMAP_TOP_RIGHT_CORNER_X: u16 = 296;
        const K_MOVING_BITMAP_TOP_RIGHT_CORNER_Y: u16 = 388;
        const K_MOVING_BITMAP_BOTTOM_RIGHT_CORNER_X: u16 = 296;
        const K_MOVING_BITMAP_BOTTOM_RIGHT_CORNER_Y: u16 = 396;
        const K_MOVING_BITMAP_BOTTOM_LEFT_CORNER_X: u16 = 288;
        const K_MOVING_BITMAP_BOTTOM_LEFT_CORNER_Y: u16 = 396;
        const K_MOVING_BITMAP_TOP_EDGE_X: u16 = 304;
        const K_MOVING_BITMAP_TOP_EDGE_Y: u16 = 396;
        const K_MOVING_BITMAP_RIGHT_EDGE_X: u16 = 304;
        const K_MOVING_BITMAP_RIGHT_EDGE_Y: u16 = 388;
        const K_MOVING_BITMAP_BOTTOM_EDGE_X: u16 = 304;
        const K_MOVING_BITMAP_BOTTOM_EDGE_Y: u16 = 396;
        const K_MOVING_BITMAP_LEFT_EDGE_X: u16 = 312;
        const K_MOVING_BITMAP_LEFT_EDGE_Y: u16 = 388;

        // Draws top-left corner
        for y in 0..K_LEVEL_EDGE_SIZE {
            for x in 0..K_LEVEL_EDGE_SIZE {
                let src_address = (K_MOVING_BITMAP_TOP_LEFT_CORNER_Y as usize + y)
                    * K_MOVING_BITMAP_WIDTH
                    + K_MOVING_BITMAP_TOP_LEFT_CORNER_X as usize
                    + x;
                let dst_address = (y * K_LEVEL_BITMAP_WIDTH) + x;
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Draws top edge
        for y in 0..K_LEVEL_EDGE_SIZE {
            for x in (K_LEVEL_EDGE_SIZE - 1)..(K_LEVEL_BITMAP_WIDTH - K_LEVEL_EDGE_SIZE) {
                let src_address = (K_MOVING_BITMAP_TOP_EDGE_Y as usize + y) * K_MOVING_BITMAP_WIDTH
                    + K_MOVING_BITMAP_TOP_EDGE_X as usize
                    + (x % K_LEVEL_EDGE_SIZE);
                let dst_address = (y * K_LEVEL_BITMAP_WIDTH) + x;
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Top-right corner
        for y in 0..K_LEVEL_EDGE_SIZE as i32 {
            for x in num_iter::range_step(
                (K_LEVEL_BITMAP_WIDTH - 1) as i32,
                (K_LEVEL_BITMAP_WIDTH - K_LEVEL_EDGE_SIZE) as i32,
                -1,
            ) {
                let src_x = x - K_LEVEL_BITMAP_WIDTH as i32 + K_LEVEL_EDGE_SIZE as i32;
                let src_address = ((K_MOVING_BITMAP_TOP_RIGHT_CORNER_Y as i32 + y)
                    * (K_MOVING_BITMAP_WIDTH as i32)
                    + (K_MOVING_BITMAP_TOP_RIGHT_CORNER_X as i32)
                    + src_x) as usize;
                let dst_address = ((y * K_LEVEL_BITMAP_WIDTH as i32) + x) as usize;
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Right edge
        for y in
            ((K_LEVEL_EDGE_SIZE - 1) as i32)..((K_LEVEL_BITMAP_HEIGHT - K_LEVEL_EDGE_SIZE) as i32)
        {
            for x in num_iter::range_step_inclusive(
                (K_LEVEL_BITMAP_WIDTH - 1) as i32,
                (K_LEVEL_BITMAP_WIDTH - K_LEVEL_EDGE_SIZE) as i32,
                -1,
            ) {
                let src_x = x - K_LEVEL_BITMAP_WIDTH as i32 + K_LEVEL_EDGE_SIZE as i32;
                let src_y = y % (K_LEVEL_EDGE_SIZE as i32);
                let src_address = ((K_MOVING_BITMAP_RIGHT_EDGE_Y as i32 + src_y)
                    * (K_MOVING_BITMAP_WIDTH as i32)
                    + K_MOVING_BITMAP_RIGHT_EDGE_X as i32
                    + src_x) as usize;
                let dst_address = ((y * K_LEVEL_BITMAP_WIDTH as i32) + x) as usize;
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Bottom-right corner
        for y in num_iter::range_step_inclusive(
            (K_LEVEL_BITMAP_HEIGHT - 1) as i32,
            (K_LEVEL_BITMAP_HEIGHT - K_LEVEL_EDGE_SIZE) as i32,
            -1,
        ) {
            for x in num_iter::range_step_inclusive(
                (K_LEVEL_BITMAP_WIDTH - 1) as i32,
                (K_LEVEL_BITMAP_WIDTH - K_LEVEL_EDGE_SIZE) as i32,
                -1,
            ) {
                let src_x = x - K_LEVEL_BITMAP_WIDTH as i32 + K_LEVEL_EDGE_SIZE as i32;
                let src_y = y - K_LEVEL_BITMAP_HEIGHT as i32 + K_LEVEL_EDGE_SIZE as i32;
                let src_address = ((K_MOVING_BITMAP_BOTTOM_RIGHT_CORNER_Y as i32 + src_y)
                    * (K_MOVING_BITMAP_WIDTH as i32)
                    + (K_MOVING_BITMAP_BOTTOM_RIGHT_CORNER_X as i32)
                    + src_x) as usize;
                let dst_address = ((y * K_LEVEL_BITMAP_WIDTH as i32) + x) as usize;
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Bottom edge
        for y in num_iter::range_step_inclusive(
            (K_LEVEL_BITMAP_HEIGHT - 1) as i32,
            (K_LEVEL_BITMAP_HEIGHT - K_LEVEL_EDGE_SIZE) as i32,
            -1,
        ) {
            for x in ((K_LEVEL_EDGE_SIZE - 1) as i32)
                ..((K_LEVEL_BITMAP_WIDTH - K_LEVEL_EDGE_SIZE) as i32)
            {
                let src_x = x % K_LEVEL_EDGE_SIZE as i32;
                let src_y = y - K_LEVEL_BITMAP_HEIGHT as i32 + K_LEVEL_EDGE_SIZE as i32;
                let src_address = ((K_MOVING_BITMAP_BOTTOM_EDGE_Y as i32 + src_y)
                    * (K_MOVING_BITMAP_WIDTH as i32)
                    + (K_MOVING_BITMAP_BOTTOM_EDGE_X as i32)
                    + src_x) as usize;
                let dst_address = ((y * K_LEVEL_BITMAP_WIDTH as i32) + x) as usize;
                assert!(dst_address < (K_LEVEL_BITMAP_WIDTH * K_LEVEL_BITMAP_HEIGHT) as usize);
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Draws left edge
        for y in
            ((K_LEVEL_EDGE_SIZE - 1) as i32)..((K_LEVEL_BITMAP_HEIGHT - K_LEVEL_EDGE_SIZE) as i32)
        {
            for x in 0..K_LEVEL_EDGE_SIZE as i32 {
                let src_y = y % K_LEVEL_EDGE_SIZE as i32;

                let src_address = ((K_MOVING_BITMAP_LEFT_EDGE_Y as i32 + src_y)
                    * (K_MOVING_BITMAP_WIDTH as i32)
                    + (K_MOVING_BITMAP_LEFT_EDGE_X as i32)
                    + x) as usize;
                let dst_address = ((y * K_LEVEL_BITMAP_WIDTH as i32) + x) as usize;
                assert!(dst_address < K_LEVEL_BITMAP_WIDTH * K_LEVEL_BITMAP_HEIGHT);
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        // Bottom-left corner

        for y in num_iter::range_step_inclusive(
            (K_LEVEL_BITMAP_HEIGHT - 1) as i32,
            (K_LEVEL_BITMAP_HEIGHT - K_LEVEL_EDGE_SIZE) as i32,
            -1,
        ) {
            for x in 0..K_LEVEL_EDGE_SIZE as i32 {
                let src_y = y - K_LEVEL_BITMAP_HEIGHT as i32 + K_LEVEL_EDGE_SIZE as i32;
                let src_address = ((K_MOVING_BITMAP_BOTTOM_LEFT_CORNER_Y as i32 + src_y)
                    * (K_MOVING_BITMAP_WIDTH as i32)
                    + (K_MOVING_BITMAP_BOTTOM_LEFT_CORNER_X as i32)
                    + x) as usize;
                let dst_address = ((y * K_LEVEL_BITMAP_WIDTH as i32) + x) as usize;
                assert!(dst_address < (K_LEVEL_BITMAP_WIDTH * K_LEVEL_BITMAP_HEIGHT) as usize);
                let color = self
                    .graphics
                    .get_pixel_from_bitmap(BitmapType::MovingDecoded, src_address as usize);
                self.graphics
                    .set_pixel(DestinationSurface::Level, dst_address as usize, color);
            }
        }

        for tile_y in 1..(K_LEVEL_HEIGHT - 1) {
            for tile_x in 1..(K_LEVEL_WIDTH - 1) {
                let bitmap_tile_x = tile_x - 1;
                let bitmap_tile_y = tile_y - 1;

                let start_dst_x = K_LEVEL_EDGE_SIZE as usize + bitmap_tile_x * K_TILE_SIZE as usize;
                let start_dst_y = K_LEVEL_EDGE_SIZE as usize + bitmap_tile_y * K_TILE_SIZE as usize;
                let mut tile_value =
                    self.states.g_current_level_state[tile_y * K_LEVEL_WIDTH + tile_x].tile;

                // Tile values greater than the official ones (including 40, the invisible wall) will be rendered as empty
                // spaces, to prevent issues even with custom graphics.
                //
                if tile_value >= LevelTileType::Count as u8 {
                    tile_value = LevelTileType::Space as u8;
                }

                let start_src_x = tile_value as usize * K_TILE_SIZE;

                for y in 0..K_TILE_SIZE {
                    for x in 0..K_TILE_SIZE {
                        let dst_address =
                            (start_dst_y + y) * (K_LEVEL_BITMAP_WIDTH) + (start_dst_x) + x;
                        let src_address = (y * (K_FIXED_BITMAP_WIDTH)) + start_src_x + x;
                        let color = self
                            .graphics
                            .get_pixel_from_bitmap(BitmapType::FixedDecoded, src_address as usize);
                        self.graphics.set_pixel(
                            DestinationSurface::Level,
                            dst_address as usize,
                            color,
                        );
                    }
                }
            }
        }
    }

    fn draw_game_panel(&mut self) {
        if self.graphics.fast_mode != FastModeType::Ultra {
            self.graphics.clear_game_panel();
        }
        self.draw_game_panel_text();
    }

    fn draw_game_panel_text(&mut self) {
        if self.graphics.fast_mode == FastModeType::Ultra {
            return;
        }

        if false
        //gIsRecordingDemo != 0
        // Recording demo?
        { /*
             self.graphics.draw_text_with_chars8_font_to_game_panel(
                 72,
                 3,
                 8,
                 "  DEMO  ".to_string(),
             );
             self.graphics.draw_text_with_chars8_font_to_game_panel(
                 16,
                 14,
                 8,
                 gCurrentDemoLevelName,
             );
             self.graphics.draw_text_with_chars8_font_to_game_panel(
                 64,
                 14,
                 8,
                 gRecordingDemoMessage,
             );*/
        } else if self.g_is_playing_demo
        // Playing demo?
        {
            let demo_level_name = self.level_manager.g_level_list_data
                [self.demo_manager.g_selected_original_demo_level_number]
                .name
                .clone();
            let demo_level_number = format!(
                "{:03}",
                self.demo_manager.g_selected_original_demo_level_number
            );

            self.graphics.draw_text_with_chars8_font_to_game_panel(
                72,
                3,
                8,
                "  DEMO  ".to_string(),
            );
            self.graphics
                .draw_text_with_chars8_font_to_game_panel(16, 14, 8, demo_level_number);
            self.graphics
                .draw_text_with_chars8_font_to_game_panel(64, 14, 8, demo_level_name);
        } else {
            self.graphics.draw_text_with_chars8_font_to_game_panel(
                72,
                3,
                6,
                self.states.g_player_name.clone(),
            );
            self.graphics.draw_text_with_chars8_font_to_game_panel(
                16,
                14,
                8,
                self.states
                    .g_current_level_name
                    .get(0..3)
                    .unwrap()
                    .to_string(),
            );
            self.graphics.draw_text_with_chars8_font_to_game_panel(
                64,
                14,
                8,
                self.states
                    .g_current_level_name
                    .get(4..)
                    .unwrap()
                    .to_string(),
            );
        }

        self.draw_number_of_remaining_infotrons();
        self.draw_game_time();
    }

    fn draw_number_of_remaining_infotrons(&mut self) {
        if self.graphics.fast_mode == FastModeType::Ultra {
            return;
        }

        if self.states.g_number_of_remaining_infotrons < 1 {
            self.states.g_number_of_remaining_infotrons = 0; // WTF? Can this be negative? In theory not...
        }

        let color = if self.states.g_number_of_remaining_infotrons == 0 {
            6
        } else {
            8
        };

        self.graphics.draw_text_with_chars8_font_to_game_panel(
            272,
            14,
            color,
            format!("{:03}", self.states.g_number_of_remaining_infotrons),
        );
    }

    fn draw_game_time(&mut self) {
        if self.graphics.fast_mode == FastModeType::Ultra {
            return;
        }

        // Only the 2 last digits will be printed, hence why it will be used with &number[1] everywhere
        if (self.states.g_last_drawn_minutes_and_seconds & 0xFF) as u8 != self.states.g_game_seconds
        // byte
        {
            self.states.g_last_drawn_minutes_and_seconds =
                (self.states.g_last_drawn_minutes_and_seconds & 0xFF00)
                    + (self.states.g_game_seconds as u16); // byte

            self.graphics.draw_text_with_chars8_font_to_game_panel(
                208,
                3,
                6,
                format!("{:02}", self.states.g_game_seconds),
            ); // seconds
        }

        if (self.states.g_last_drawn_minutes_and_seconds >> 8) as u8 != self.states.g_game_minutes
        // byte
        {
            self.states.g_last_drawn_minutes_and_seconds = ((self.states.g_game_minutes as u16)
                << 8)
                + (self.states.g_last_drawn_minutes_and_seconds & 0x00FF); // byte
            self.graphics.draw_text_with_chars8_font_to_game_panel(
                208,
                3,
                6,
                format!("{:02}", self.states.g_game_minutes),
            );
        }

        if self.states.g_last_drawn_hours != self.states.g_game_hours {
            self.states.g_last_drawn_hours = self.states.g_game_hours;
            self.graphics.draw_text_with_chars8_font_to_game_panel(
                208,
                3,
                6,
                format!("{:02}", self.states.g_game_hours),
            );
        }
    }

    fn convert_to_easy_tiles(&mut self) -> u16 {
        let mut number_of_infotrons = 0;
        let mut number_of_something = 0; // this is bx, just counts the number of tiles so technically is same as cx at this point… probably a return value but I don't see it used anywhere???

        for i in 0..K_LEVEL_SIZE {
            let current_tile = i;
            number_of_something += 1;

            if self.states.g_current_level_state[current_tile].tile == 0xf1 {
                self.states.g_current_level_state[current_tile].tile = LevelTileType::Explosion as u8;
                continue; // jmp short loc_4A3B0
            }

            if self.g_is_game_busy != true {
                if self.states.g_current_level_state[current_tile].tile == LevelTileType::Infotron as u8 {
                    number_of_infotrons += 1;
                    continue; // jmp short loc_4A3B0
                }
            }
            // TODO: what are these gIsGameBusy for??
            if self.g_is_game_busy == true
                || self.states.g_current_level_state[current_tile].state != 0
                || self.states.g_current_level_state[current_tile].tile != LevelTileType::SnikSnak as u8
            //jz  short loc_4A34B
            {
                if self.g_is_game_busy == true
                    || self.states.g_current_level_state[current_tile].state != 0
                    || self.states.g_current_level_state[current_tile].tile != LevelTileType::Electron as u8
                //jz  short loc_4A379
                {
                    if (self.states.g_current_level_state[current_tile].state == 0
                        && self.states.g_current_level_state[current_tile].tile == LevelTileType::HorizontalChipLeft as u8)
                        || (self.states.g_current_level_state[current_tile].state == 0
                            && self.states.g_current_level_state[current_tile].tile == LevelTileType::HorizontalChipRight as u8)
                        || (self.states.g_current_level_state[current_tile].state == 0
                            && self.states.g_current_level_state[current_tile].tile == LevelTileType::HorizontalChipTop as u8)
                        || (self.states.g_current_level_state[current_tile].state == 0
                            && self.states.g_current_level_state[current_tile].tile == LevelTileType::HorizontalChipBottom as u8)
                    {
                        self.states.g_current_level_state[current_tile].tile = LevelTileType::Chip as u8; // mov word ptr [si], 5
                        self.states.g_current_level_state[current_tile].state = 0;
                        continue; // jmp short loc_4A3B0
                    }
                    if self.states.g_current_level_state[current_tile].state == 0
                        && self.states.g_current_level_state[current_tile].tile >= LevelTileType::Hardware2 as u8
                        && self.states.g_current_level_state[current_tile].tile <= LevelTileType::Hardware11 as u8
                    {
                        self.states.g_current_level_state[current_tile].tile = LevelTileType::Hardware as u8; // mov word ptr [si], 6
                        self.states.g_current_level_state[current_tile].state = 0;
                        continue; // jmp short loc_4A3B0
                    }

                    if self.states.g_current_level_state[current_tile].state == 0
                        && self.states.g_current_level_state[current_tile].tile >= LevelTileType::SportRight as u8
                        && self.states.g_current_level_state[current_tile].tile <= LevelTileType::SportUp as u8
                    {
                        self.states.g_current_level_state[current_tile].tile -= 4; // Converts Sport[Direction] to Port[Direction]
                        self.states.g_current_level_state[current_tile].state = 1;
                        continue;
                    }

                    continue;
                }
            }

            let left_tile = i-1;
            let above_tile = i-K_LEVEL_WIDTH;
            let right_tile = i + 1;

            if self.states.g_current_level_state[current_tile].state != 0 || self.states.g_current_level_state[current_tile].tile != LevelTileType::Electron as u8
            //jz  short loc_4A379
            {
                if self.states.g_current_level_state[left_tile].tile == (LevelTileType::Space as u8) && self.states.g_current_level_state[left_tile].state == 0
                //cmp word ptr [si-2], 0
                {
                    self.states.g_current_level_state[current_tile].state = 1;
                    continue; // jmp short loc_4A3B0
                }
                if self.states.g_current_level_state[above_tile].tile == (LevelTileType::Space as u8) && self.states.g_current_level_state[above_tile].state == 0
                //cmp word ptr [si-78h], 0
                {
                    self.states.g_current_level_state[above_tile].state = 0x10;
                    self.states.g_current_level_state[above_tile].tile = LevelTileType::SnikSnak as u8;
                    // mov word ptr [si], 0FFFFh
                    self.states.g_current_level_state[current_tile].state = 0xFF;
                    self.states.g_current_level_state[current_tile].tile = 0xFF;
                    continue; // jmp short loc_4A3B0
                }
                if self.states.g_current_level_state[right_tile].tile == (LevelTileType::Space as u8) && self.states.g_current_level_state[right_tile].state == 0
                //cmp word ptr [si+2], 0
                {
                    self.states.g_current_level_state[right_tile].state = 0x28;
                    self.states.g_current_level_state[right_tile].tile = LevelTileType::SnikSnak as u8;
                    self.states.g_current_level_state[current_tile].state = 0xFF;
                    self.states.g_current_level_state[current_tile].tile = 0xFF;
                    continue; // jmp short loc_4A3B0
                }

                continue;
            }
            if self.states.g_current_level_state[left_tile].tile == LevelTileType::Space as u8 && self.states.g_current_level_state[left_tile].state == 0
            //cmp word ptr [si-2], 0
            {
                self.states.g_current_level_state[current_tile].state = 1; //mov byte ptr [si+1], 1
                continue; // jmp short loc_4A3B0
            }
            if self.states.g_current_level_state[above_tile].tile == LevelTileType::Space as u8 && self.states.g_current_level_state[above_tile].state == 0
            //cmp word ptr [si-78h], 0
            {
                // mov word ptr [si-78h], 1018h
                self.states.g_current_level_state[above_tile].state = 0x10;
                self.states.g_current_level_state[above_tile].tile = LevelTileType::Electron as u8;
                // mov word ptr [si], 0FFFFh
                self.states.g_current_level_state[current_tile].state = 0xFF;
                self.states.g_current_level_state[current_tile].tile = 0xFF;
                continue; // jmp short loc_4A3B0
            }
            if self.states.g_current_level_state[right_tile].tile == LevelTileType::Space as u8 && self.states.g_current_level_state[right_tile].state == 0
            //cmp word ptr [si+2], 0
            {
                self.states.g_current_level_state[right_tile].state = 0x28;
                self.states.g_current_level_state[right_tile].tile = LevelTileType::Electron as u8;
                self.states.g_current_level_state[current_tile].state = 0xFF;
                self.states.g_current_level_state[current_tile].tile = 0xFF;
                continue; // jmp short loc_4A3B0
            }
        }

        number_of_infotrons
    }

    fn find_murphy(&mut self) {
        for i in 0..K_LEVEL_SIZE {
            if self.states.g_current_level.tiles[i] == LevelTileType::Murphy as u8 {
                self.states.g_murphy_location = i;
                break;
            }
        }

        self.scroll_to_murphy();
    }

    fn scroll_to_murphy(&mut self) {
        // Parameters:
        // - si: murphy location * 2
        // - al: murphy location

        self.states.g_murphy_tile_x = self.states.g_murphy_location % K_LEVEL_WIDTH; // stores X coord
        self.states.g_murphy_tile_y = self.states.g_murphy_location / K_LEVEL_WIDTH; // stores Y coord

        self.states.g_murphy_position_x = self.states.g_murphy_tile_x * K_TILE_SIZE;
        self.states.g_murphy_position_y = self.states.g_murphy_tile_y * K_TILE_SIZE;

        self.graphics
            .draw_moving_frame(304, 132, self.states.g_murphy_location);
        self.update_scroll_offset();

        self.graphics.video_loop();
    }

    fn reset_number_of_infotrons(&mut self, number_of_infotrons_found_in_level: u16) {
        // In the original game, the number of infotrons found in a level is stored in a 2-bytes variable,
        // however, when stored for its use in the game, it's stored in a 1-byte variable.
        //
        let mut number_of_infotrons = (number_of_infotrons_found_in_level & 0xFF) as u8;
        if self.states.g_number_of_info_trons != 0 {
            number_of_infotrons = self.states.g_number_of_info_trons;
        }

        self.states.g_number_of_remaining_infotrons = number_of_infotrons;

        self.states.g_total_number_of_infotrons = number_of_infotrons;
        self.draw_number_of_remaining_infotrons();
    }
    fn update_scroll_offset(&mut self) {
        let mut random_number = 0;

        // This random number is used to generate the shaking effect on explosions.
        // The original game generates this random number here for _every_ explosion, even if
        // normally only Murphy's explosion will make the screen shake. However it's necessary
        // to do this here to make sure the right sequence of random numbers is generated when
        // there are explosions in the level.
        //
        if self.states.g_is_explosion_started {
            random_number = self.generate_random_number();
        }

        let mut scroll_x = self.states.g_murphy_position_x as i32;
        let mut scroll_y = self.states.g_murphy_position_y as i32;
        scroll_x -= K_SCREEN_WIDTH as i32 / 2; // 152
        if scroll_x < 0 {
            scroll_x = 0;
        }

        let mut max_scroll_x = (K_LEVEL_BITMAP_WIDTH - K_SCREEN_WIDTH) as i32;
        if scroll_x > max_scroll_x
        // 624
        {
            scroll_x = max_scroll_x; // 624
        }

        if self.states.g_should_show_game_panel == false {
            scroll_y -= (K_SCREEN_HEIGHT / 2) as i32;
        } else {
            scroll_y -= ((K_SCREEN_HEIGHT - K_PANEL_BITMAP_HEIGHT) / 2) as i32;
        }

        if scroll_y < 0 {
            scroll_y = 0;
        }

        let mut max_scroll_y = 0;

        if self.states.g_should_show_game_panel == false {
            max_scroll_y = (K_LEVEL_BITMAP_HEIGHT - K_SCREEN_HEIGHT) as i32;
            if scroll_y > max_scroll_y {
                scroll_y = max_scroll_y;
            }
        } else {
            max_scroll_y = (K_LEVEL_BITMAP_HEIGHT - K_SCREEN_HEIGHT + K_PANEL_BITMAP_HEIGHT) as i32;
            if scroll_y > max_scroll_y {
                scroll_y = max_scroll_y;
            }
        }

        if self.states.g_is_move_scroll_mode_enabled == false
            || self.keyboard.borrow().g_is_numpad_5_key_pressed
        {
            self.states.g_murphy_scroll_offset_x = scroll_x;
            self.states.g_murphy_scroll_offset_y = scroll_y;
        } else {
            scroll_x = self.states.g_murphy_scroll_offset_x;
            scroll_y = self.states.g_murphy_scroll_offset_y;

            let mut additional_scroll_x = scroll_x;
            scroll_x += self.graphics.g_additional_scroll_offset_x;
            if scroll_x < 0 {
                scroll_x = 0;
            } else {
                if scroll_x > max_scroll_x {
                    scroll_x = max_scroll_x;
                }
            }

            additional_scroll_x -= scroll_x;
            additional_scroll_x = -additional_scroll_x;
            self.graphics.g_additional_scroll_offset_x = additional_scroll_x;

            let mut additional_scroll_y = scroll_y;
            scroll_y += self.graphics.g_additional_scroll_offset_y;
            if scroll_y < 0
            // in asm there wasn't a explicit "cmp", just the "add" above
            {
                scroll_y = 0;
            } else {
                if scroll_y > max_scroll_y
                // 168
                {
                    scroll_y = max_scroll_y; // 168
                }
            }

            additional_scroll_y -= scroll_y;
            additional_scroll_y = -additional_scroll_y;
            self.graphics.g_additional_scroll_offset_y = additional_scroll_y;
        }

        // This makes the screen shake on an explosion
        if self.states.g_should_shake_with_all_explosions != false // could be == true ? or should not be bool ?
            || (self.states.g_shake_with_explosion_disabled == false
                && (self.states.g_quit_level_countdown & 0xFF) != 0)
        {
            random_number = random_number & 0x101;

            let scroll_shake_yoffset = (random_number >> 8) as i32;
            let mut scroll_shake_xoffset = (random_number & 0xFF) as i32;

            scroll_y += scroll_shake_yoffset;
            if scroll_x > 0x13C
            // 316
            {
                scroll_shake_xoffset = -scroll_shake_xoffset;
            }

            scroll_x += scroll_shake_xoffset;
        }

        self.graphics.g_scroll_offset_x = scroll_x;
        self.graphics.g_scroll_offset_y = scroll_y;
    }
}
