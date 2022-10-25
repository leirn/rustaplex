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

pub const VERSION_STRING: &str = "0.1";
pub const GAME_NAME: &str = "RUSTAPLEX";

pub const WINDOW_TITLE: &str = "Rustaplex";

pub const K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING: usize = 344;
pub const K_LEVEL_DATA_LENGTH: usize = 1525;

pub const K_LEVEL_WIDTH: usize = 60;
pub const K_LEVEL_HEIGHT: usize = 24;
pub const K_LEVEL_SIZE: usize = K_LEVEL_WIDTH * K_LEVEL_HEIGHT;

pub const K_NUMBER_OF_LEVEL: usize = 111;
pub const K_NUMBER_OF_LEVEL_WITH_PADDING: usize = K_NUMBER_OF_LEVEL + 5;
pub const K_FIRST_LEVEL_INDEX: usize = 2;
pub const K_LAST_LEVEL_INDEX: usize = K_FIRST_LEVEL_INDEX + K_NUMBER_OF_LEVEL;
pub const K_LIST_LEVEL_NAME_LENGTH: usize = 28; // In the list of levels, every level is 28 bytes long and looks like "001
pub const K_LEVEL_NAME_LENGTH: usize = 24;

pub const K_LEVEL_LIST_DATA_LENGTH: usize = K_NUMBER_OF_LEVEL * K_LIST_LEVEL_NAME_LENGTH;

pub const K_CONFIG_DATA_LENGTH: usize = 4;

pub const RESSOURCES_PATH: &str = "resources";
pub const K_BASE_AUDIO_FOLDER: &str = "audio";

// Général file names
pub const G_CONFIG_FILE_NAME: &str = "SUPAPLEX.CFG";
pub const G_LEVELS_LST_FILENAME: &str = "LEVEL.LST";
pub const G_PLAYERS_LST_FILENAME: &str = "PLAYER.LST";
pub const G_HALL_OF_FAME_LST_FILENAME: &str = "HALLFAME.LST";
pub const G_SAVEGAME_SAV_FILENAME: &str = "SAVEGAME.SAV";

// Graphic file names
pub const G_MENU_DAT_FILENAME: &str = "MENU.DAT";
pub const G_MOVING_DAT_FILENAME: &str = "MOVING.DAT";
pub const G_FIXED_DAT_FILENAME: &str = "FIXED.DAT";
pub const G_PANEL_DAT_FILENAME: &str = "PANEL.DAT";
pub const G_BACK_DAT_FILENAME: &str = "BACK.DAT";
pub const G_CHARS6_DAT_FILENAME: &str = "CHARS6.DAT";
pub const G_CHARS8_DAT_FILENAME: &str = "CHARS8.DAT";
pub const G_TITLE_DAT_FILENAME: &str = "TITLE.DAT";
pub const G_TITLE1_DAT_FILENAME: &str = "TITLE1.DAT";
pub const G_TITLE2_DAT_FILENAME: &str = "TITLE2.DAT";
pub const G_GFX_DAT_FILENAME: &str = "GFX.DAT";
pub const G_PALETTES_DAT_FILENAME: &str = "PALETTES.DAT";
pub const G_CONTROLS_DAT_FILENAME: &str = "CONTROLS.DAT";

#[derive(Default, Copy, Clone)]
pub struct StatefulLevelTile {
    pub tile: u8, // of LevelTileType
    pub state: u8,
}

#[derive(Copy, Clone)]
pub enum LevelTileType {
    LevelTileTypeSpace = 0,
    LevelTileTypeZonk = 1,
    LevelTileTypeBase = 2,
    LevelTileTypeMurphy = 3,
    LevelTileTypeInfotron = 4,
    LevelTileTypeChip = 5,
    LevelTileTypeHardware = 6,
    LevelTileTypeExit = 7,
    LevelTileTypeOrangeDisk = 8,
    LevelTileTypePortRight = 9,
    LevelTileTypePortDown = 10,
    LevelTileTypePortLeft = 11,
    LevelTileTypePortUp = 12,
    LevelTileTypeSportRight = 13,
    LevelTileTypeSportDown = 14,
    LevelTileTypeSportLeft = 15,
    LevelTileTypeSportUp = 16,
    LevelTileTypeSnikSnak = 17,
    LevelTileTypeYellowDisk = 18,
    LevelTileTypeTerminal = 19,
    LevelTileTypeRedDisk = 20,
    LevelTileTypePortVertical = 21,
    LevelTileTypePortHorizontal = 22,
    LevelTileTypePort4Way = 23,
    LevelTileTypeElectron = 24,
    LevelTileTypeBug = 25,
    LevelTileTypeHorizontalChipLeft = 26,
    LevelTileTypeHorizontalChipRight = 27,
    LevelTileTypeHardware2 = 28,
    LevelTileTypeHardware3 = 29,
    LevelTileTypeHardware4 = 30,
    LevelTileTypeExplosion = 31,
    LevelTileTypeHardware6 = 32,
    LevelTileTypeHardware7 = 33,
    LevelTileTypeHardware8 = 34,
    LevelTileTypeHardware9 = 35,
    LevelTileTypeHardware10 = 36,
    LevelTileTypeHardware11 = 37,
    LevelTileTypeHorizontalChipTop = 38,
    LevelTileTypeHorizontalChipBottom = 39,
    LevelTileTypeCount,
}

pub struct PlayerEntry {
    pub name: String,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub level_state: [u8; K_NUMBER_OF_LEVEL], // values are PlayerLevelState
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub next_level_to_play: u8,
    pub completed_all_levels: u8, // Still not 100% sure
}

impl PlayerEntry {
    pub fn new() -> PlayerEntry {
        PlayerEntry {
            name: String::from("--------"), // Default player name
            hours: 0,
            minutes: 0,
            seconds: 0,
            level_state: [0; K_NUMBER_OF_LEVEL], // values are PlayerLevelState
            unknown1: 0,
            unknown2: 0,
            unknown3: 0,
            next_level_to_play: 0,
            completed_all_levels: 0, // Still not 100% sure
        }
    }
}

pub struct HallOfFameEntry {
    pub name: String,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl HallOfFameEntry {
    pub fn new() -> HallOfFameEntry {
        HallOfFameEntry {
            name: String::from("--------"), // Default player name
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub const K_PLAYER_ENTRY_SIZE: usize = K_PLAYER_NAME_LENGTH + 1 + 3 + K_NUMBER_OF_LEVEL + 5;
pub const K_HALL_OF_FAME_ENTRY_SIZE: usize = K_PLAYER_NAME_LENGTH + 1 + 3;
pub const K_PLAYER_NAME_LENGTH: usize = 8;
pub const K_NUMBER_OF_PLAYERS: usize = 20;
pub const K_NUMBER_OF_HALL_OF_FAME_ENTRIES: usize = 3;

#[derive(Default, Clone, Copy)]
pub struct SpecialPortInfo {
    // If (x,y) are the coordinates of a port in the field
    // and (0,0) is the top-left corner, the 16 bit value
    // here calculates as 2*(x+(y*60)).  This is twice of
    // what you may have expected: Supaplex works with a
    // game field in memory, which is 2 bytes per sprite.
    pub position: u16,

    pub gravity: u8,       // 1 = turn on, anything else (0) = turn off
    pub freeze_zonks: u8,   // 2 = turn on, anything else (0) = turn off  (1=off!)
    pub freeze_enemies: u8, // 1 = turn on, anything else (0) = turn off
    pub unused: u8,        // Doesn't matter: is ignored.
}

pub const K_LEVEL_MAX_NUMBER_OF_SPECIAL_PORTS: usize = 10;
