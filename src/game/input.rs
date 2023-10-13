use std::{cell::RefCell, rc::Rc};

use super::gamecontroller::GameController;
use super::keyboard::Keys;

pub struct Input {
    keyboard: Rc<RefCell<Keys>>,
    gamecontroller: GameController,
}

impl Input {
    pub fn new(keyboard: Rc<RefCell<Keys>>) -> Input {
        Input {
            keyboard: keyboard,
            gamecontroller: GameController::default(),
        }
    }

    fn is_up_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_up_key_pressed
            || self.gamecontroller.get_game_controller_y() < 0
    }

    pub fn is_down_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_down_key_pressed
            || self.gamecontroller.get_game_controller_y() > 0
    }

    pub fn is_left_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_left_key_pressed
            || self.gamecontroller.get_game_controller_x() < 0
    }

    pub fn is_right_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_right_key_pressed
            || self.gamecontroller.get_game_controller_x() > 0
    }

    pub fn is_action_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_space_key_pressed
            || self.gamecontroller.get_game_controller_button_x()
    }

    pub fn is_menu_back_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_escape_key_pressed
            || self.gamecontroller.get_game_controller_button_back()
    }

    pub fn is_menu_confirm_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_enter_key_pressed
            || self.gamecontroller.get_game_controller_confirm_button()
    }

    pub fn is_menu_cancel_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_escape_key_pressed
            || self.gamecontroller.get_game_controller_cancel_button()
            || self.gamecontroller.get_game_controller_button_back()
    }

    pub fn is_exit_level_button_pressed(&mut self) -> bool {
        self.gamecontroller.get_game_controller_button_back()
    }

    pub fn is_start_button_pressed(&mut self) -> bool {
        self.gamecontroller.get_game_controller_button_start()
    }

    pub fn is_pause_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_p_key_pressed
            || self.keyboard.borrow().g_is_escape_key_pressed
            || self.gamecontroller.get_game_controller_button_start()
    }

    pub fn is_toggle_game_panel_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_enter_key_pressed
            || self.gamecontroller.get_game_controller_button_y()
    }

    pub fn is_show_number_of_red_disks_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_right_shift_pressed
            || self.gamecontroller.get_game_controller_button_b()
    }

    pub fn is_increase_game_speed_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_numpad_plus_pressed
            || self
                .gamecontroller
                .get_game_controller_button_right_shoulder()
    }

    pub fn is_decrease_game_speed_button_pressed(&mut self) -> bool {
        self.keyboard.borrow().g_is_numpad_minus_pressed
            || self
                .gamecontroller
                .get_game_controller_button_left_shoulder()
    }

    pub fn is_rotate_level_set_ascending_button_pressed(&mut self) -> bool {
        self.gamecontroller
            .get_game_controller_button_right_shoulder()
    }

    pub fn is_rotate_level_set_descending_button_pressed(&mut self) -> bool {
        self.gamecontroller
            .get_game_controller_button_left_shoulder()
    }
}
