pub const K_SIZE_OF_LEVEL_STATE_PRECEDING_PADDING: usize = 344;
pub const K_LEVEL_DATA_LENGTH: usize = 1525;

pub const RESSOURCES_PATH: &str = "resources";

// Général file names
pub const G_LEVELS_DAT_FILENAME: &str = "LEVELS.DAT";
pub const G_LEVELS_LST_FILENAME: &str = "LEVEL.LST";
pub const G_PLAYERS_LST_FILENAME: &str = "PLAYER.LST";
pub const G_HALL_OF_FAME_LST_FILENAME: &str = "HALLFAME.LST";
pub const G_DEMO0_BIN_FILENAME: &str = "DEMO0.BIN";
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
