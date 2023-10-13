#[derive(Default)]
pub struct GameController {}

impl GameController {
    pub fn get_game_controller_y(&mut self) -> u8 {
        0
    }
    pub fn get_game_controller_x(&mut self) -> u8 {
        0
    }
    pub fn get_game_controller_a(&mut self) -> u8 {
        0
    }
    pub fn get_game_controller_b(&mut self) -> u8 {
        0
    }
    pub fn get_game_controller_button_a(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_b(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_y(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_x(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_back(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_confirm_button(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_cancel_button(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_start(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_right_shoulder(&mut self) -> bool {
        false
    }
    pub fn get_game_controller_button_left_shoulder(&mut self) -> bool {
        false
    }
}
