use crate::game::globals::*;

use crate::game::animation::MurphyAnimationDescriptor;
use crate::game::level::Level;

pub struct GameStates {
    pub g_current_level_state_with_padding:
        [StatefulLevelTile; K_LEVEL_DATA_LENGTH + K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING], // 0x1584
    pub g_current_level_state: StatefulLevelTile, // located at 0x1834, size is kLevelDataLength items
    pub g_explosion_timers: [u8; K_LEVEL_DATA_LENGTH], // 0x2434
    pub g_is_gravity_enabled: u8, // byte_5101C -> 1 = turn on, anything else (0) = turn off
    pub g_are_zonks_frozen: u8, // byte_51035 -> 2 = turn on, anything else (0) = turn off  (1=off!)
    pub g_number_of_info_trons: u8, // 0xd26 -> byte_51036 -> this seems to be _inside_ of fileLevelData when a level is read
    pub g_number_of_special_ports: u8, // 0xd27 -> byte_51037 this seems to be _inside_ of fileLevelData when a level is read, and it's numberOfSpecialPorts
    pub g_random_seed: u16,            // word_51076
    pub g_aux_game_seconds_20ms_accumulator: u8, // byte_510AF ->  -> accumulates game time. The total time is its value * 20ms, so when it reaches 50 it means 1 second. Used to increase the game time in the bottom panel
    pub g_game_seconds: u8,                      // byte_510B0
    pub g_game_minutes: u8,                      // byte_510B1
    pub g_game_hours: u8,                        // byte_510B2
    pub g_should_update_total_level_time: u8,    // byte_510B3
    pub g_level_failed: bool,                    // byte_510BA
    pub g_current_player_level_state: PlayerLevelState, // byte_510BB
    pub g_is_explosion_started: bool, // byte_510C0 -> Set to 1 when an explosion is just created. Set back to 0 when _any_ of the explosions on the screen disappears.
    pub g_should_show_game_panel: bool, // byte_510C1 -> 0DB1
    pub g_toggle_game_panel_key_auto_repeat_counter: u8, // byte_510C2 -> 0DB2
    pub g_murphy_tile_x: i16,         // word_510C3
    pub g_murphy_tile_y: i16,         // word_510C5
    pub g_murphy_previous_location: i16, // word_510C7
    pub g_murphy_location: i16,
    pub g_is_murphy_looking_left: bool,          // word_510CB
    pub g_murphy_yawn_and_sleep_counter: u16,    // word_510CD
    pub g_is_murphy_updated: bool,               // word_510CF
    pub g_should_kill_murphy: bool,              // word_510D1
    pub g_previous_user_input_was_none: bool, // byte_510D3 -> used to detect when to release the red disk
    pub g_are_enemies_frozen: bool, // byte_510D7 -> 1 = turn on, anything else (0) = turn off
    pub g_scratch_gravity: bool, // byte_510D8 -> not sure what scratch gravity means exactly, but can be 0 (off) or 1 (on)
    pub g_is_murphy_going_through_portal: bool, // word_510D9
    pub g_planted_red_disk_countdown: u8, // byte_510DB
    pub g_planted_red_disk_position: u16, // word_510DC
    pub g_demo_current_input_index: u16, // word_510DF
    pub g_demo_current_input: u8, // byte_510E1 -> 0xDD1
    pub g_demo_current_input_repeat_counter: u8, // -> 0xDD2 -> byte_510E2
    pub g_demo_index_or_demo_level_number: u16, // word_510E6
    pub g_murphy_position_x: u16, // word_510E8
    pub g_murphy_position_y: u16, // word_510EA
    pub g_murphy_counter_to_start_push_animation: u16, // word_510EE -> also used in the "release red disk" animation
    pub g_current_murphy_animation: MurphyAnimationDescriptor, // -> starts at 0x0DE0
    pub g_number_of_remaining_infotrons: u8,           // byte_5195A
    pub g_total_number_of_infotrons: u8,               // byte_5195B
    pub g_number_of_remaining_red_disks: u8,           // byte_5195C
    pub g_frame_counter: u16,                          // word_5195D -> 0x1268
    pub g_terminal_max_frames_to_next_scroll: u8, // byte_5196A -> this is used to make the terminals scroll their screens faster after the yellow disks have been detonated
    pub g_are_yellow_disks_detonated: u8,         // byte_5196B
    pub g_should_leave_main_menu: bool,           // word_5196C
    pub g_should_exit_level: u16,                 // word_51974
    pub g_quit_level_countdown: u16, // word_51978 -> this is a counter to end the level after certain number of iterations (to let the game progress a bit before going back to the menu)
    pub g_additional_info_in_game_panel_frame_counter: u8, // byte_5197C -> how many more frames the additional info in the game panel will be
    pub g_current_level: Level,                            // 0x988B
    pub g_is_playing_demo: bool,
    pub g_current_player_index: usize,
    pub g_current_player_padded_level_data: [u8; K_NUMBER_OF_LEVEL_WITH_PADDING],
    pub g_current_selected_level_index: u8,
    pub g_player_name: String,
    pub g_current_level_name: String,
    pub g_is_level_started_as_demo: bool,
    pub g_has_user_cheated: bool,
    pub g_new_player_entry_index: usize,
    pub g_new_player_name_length: u8,
}

impl GameStates {
    pub fn new() -> GameStates {
        GameStates {
            g_current_level_state_with_padding: [StatefulLevelTile::default();
                K_LEVEL_DATA_LENGTH + K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING],
            g_current_level_state: StatefulLevelTile::default(),
            g_explosion_timers: [0; K_LEVEL_DATA_LENGTH],
            g_is_gravity_enabled: 0,
            g_are_zonks_frozen: 0,
            g_number_of_info_trons: 0,
            g_number_of_special_ports: 0,
            g_random_seed: 0,
            g_aux_game_seconds_20ms_accumulator: 0,
            g_game_seconds: 0,
            g_game_minutes: 0,
            g_game_hours: 0,
            g_should_update_total_level_time: 0,
            g_level_failed: false,
            g_current_player_level_state: PlayerLevelState::NotCompleted,
            g_is_explosion_started: false,
            g_should_show_game_panel: false,
            g_toggle_game_panel_key_auto_repeat_counter: 0,
            g_murphy_tile_x: 0,
            g_murphy_tile_y: 0,
            g_murphy_previous_location: 0,
            g_murphy_location: 0,
            g_is_murphy_looking_left: false,
            g_murphy_yawn_and_sleep_counter: 0,
            g_is_murphy_updated: false,
            g_should_kill_murphy: false,
            g_previous_user_input_was_none: false,
            g_are_enemies_frozen: false,
            g_scratch_gravity: false,
            g_is_murphy_going_through_portal: false,
            g_planted_red_disk_countdown: 0,
            g_planted_red_disk_position: 0,
            g_demo_current_input_index: 0,
            g_demo_current_input: 0,
            g_demo_current_input_repeat_counter: 0,
            g_demo_index_or_demo_level_number: 0,
            g_murphy_position_x: 0,
            g_murphy_position_y: 0,
            g_murphy_counter_to_start_push_animation: 0,
            g_current_murphy_animation: MurphyAnimationDescriptor::default(),
            g_number_of_remaining_infotrons: 0,
            g_total_number_of_infotrons: 0,
            g_number_of_remaining_red_disks: 0,
            g_frame_counter: 0,
            g_terminal_max_frames_to_next_scroll: 0,
            g_are_yellow_disks_detonated: 0,
            g_should_leave_main_menu: false,
            g_should_exit_level: 0,
            g_quit_level_countdown: 0,
            g_additional_info_in_game_panel_frame_counter: 0,
            g_current_level: Level::new(),
            g_is_playing_demo: false,
            g_current_player_index: 0,

            g_current_player_padded_level_data: [0; K_NUMBER_OF_LEVEL_WITH_PADDING],
            g_current_selected_level_index: 0,
            g_player_name: String::new(),
            g_current_level_name: String::new(),
            g_is_level_started_as_demo: false,
            g_has_user_cheated: false,

            g_new_player_entry_index: 0,
            g_new_player_name_length: 0,
        }
    }

    pub fn get_g_current_player_level_data(&self, index: usize) -> u8 {
        self.g_current_player_padded_level_data[index + K_FIRST_LEVEL_INDEX]
    }

    pub fn set_g_current_player_level_data(&mut self, index: usize, value: u8) {
        self.g_current_player_padded_level_data[index + K_FIRST_LEVEL_INDEX] = value;
    }
}
