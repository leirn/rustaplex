use std::ops::AddAssign;

use sdl2::keyboard::{PressedScancodeIterator, Scancode};

#[derive(Clone, Default)]
pub struct Keys {
    pub g_is_up_key_pressed: bool,
    pub g_is_down_key_pressed: bool,
    pub g_is_left_key_pressed: bool,
    pub g_is_right_key_pressed: bool,
    pub g_is_insert_key_pressed: bool,
    pub g_is_end_key_pressed: bool,
    pub g_is_av_page_key_pressed: bool,
    pub g_is_re_page_key_pressed: bool,
    pub g_is_numpad_5_key_pressed: bool,
    pub g_is_home_key_pressed: bool,
    pub g_is_del_key_pressed: bool,
    pub g_is_numpad_divide_pressed: bool,
    pub g_is_numpad_multiply_pressed: bool,
    pub g_is_numpad_plus_pressed: bool,
    pub g_is_numpad_minus_pressed: bool,
    pub g_is_num_lock_pressed: bool,
    pub g_is_scroll_lock_pressed: bool,
    pub g_is_q_key_pressed: bool,
    pub g_is_w_key_pressed: bool,
    pub g_is_r_key_pressed: bool,
    pub g_is_p_key_pressed: bool,
    pub g_is_s_key_pressed: bool,
    pub g_is_d_key_pressed: bool,
    pub g_is_h_key_pressed: bool,
    pub g_is_j_key_pressed: bool,
    pub g_is_l_key_pressed: bool,
    pub g_is_z_key_pressed: bool,
    pub g_is_x_key_pressed: bool,
    pub g_is_c_key_pressed: bool,
    pub g_is_b_key_pressed: bool,
    pub g_is_m_key_pressed: bool,
    pub g_is_1_key_pressed: bool,
    pub g_is_2_key_pressed: bool,
    pub g_is_3_key_pressed: bool,
    pub g_is_4_key_pressed: bool,
    pub g_is_5_key_pressed: bool,
    pub g_is_6_key_pressed: bool,
    pub g_is_7_key_pressed: bool,
    pub g_is_8_key_pressed: bool,
    pub g_is_9_key_pressed: bool,
    pub g_is_0_key_pressed: bool,
    pub g_is_f1_key_pressed: bool,
    pub g_is_f2_key_pressed: bool,
    pub g_is_f3_key_pressed: bool,
    pub g_is_f4_key_pressed: bool,
    pub g_is_f5_key_pressed: bool,
    pub g_is_f6_key_pressed: bool,
    pub g_is_f7_key_pressed: bool,
    pub g_is_f8_key_pressed: bool,
    pub g_is_f9_key_pressed: bool,
    pub g_is_f10_key_pressed: bool,
    pub g_is_f11_key_pressed: bool,
    pub g_is_f12_key_pressed: bool,
    pub g_is_minus_key_pressed: bool,
    pub g_is_equals_key_pressed: bool,
    pub g_is_escape_key_pressed: bool,
    pub g_is_space_key_pressed: bool,
    pub g_is_enter_key_pressed: bool,
    pub g_is_left_control_key_pressed: bool,
    pub g_is_left_alt_pressed: bool,
    pub g_is_left_shift_pressed: bool,
    pub g_is_right_shift_pressed: bool,
    pub g_current_user_input: UserInput,
    pub g_key_pressed: Option<Scancode>,
}

impl Keys {
    fn reset_pressed_keys(&mut self) {
        self.g_is_up_key_pressed = false;
        self.g_is_down_key_pressed = false;
        self.g_is_left_key_pressed = false;
        self.g_is_right_key_pressed = false;
        self.g_is_insert_key_pressed = false;
        self.g_is_end_key_pressed = false;
        self.g_is_av_page_key_pressed = false;
        self.g_is_re_page_key_pressed = false;
        self.g_is_numpad_5_key_pressed = false;
        self.g_is_home_key_pressed = false;
        self.g_is_del_key_pressed = false;
        self.g_is_numpad_divide_pressed = false;
        self.g_is_numpad_multiply_pressed = false;
        self.g_is_numpad_plus_pressed = false;
        self.g_is_numpad_minus_pressed = false;
        self.g_is_num_lock_pressed = false;
        self.g_is_scroll_lock_pressed = false;
        self.g_is_q_key_pressed = false;
        self.g_is_w_key_pressed = false;
        self.g_is_r_key_pressed = false;
        self.g_is_p_key_pressed = false;
        self.g_is_s_key_pressed = false;
        self.g_is_d_key_pressed = false;
        self.g_is_h_key_pressed = false;
        self.g_is_j_key_pressed = false;
        self.g_is_l_key_pressed = false;
        self.g_is_z_key_pressed = false;
        self.g_is_x_key_pressed = false;
        self.g_is_c_key_pressed = false;
        self.g_is_b_key_pressed = false;
        self.g_is_m_key_pressed = false;
        self.g_is_1_key_pressed = false;
        self.g_is_2_key_pressed = false;
        self.g_is_3_key_pressed = false;
        self.g_is_4_key_pressed = false;
        self.g_is_5_key_pressed = false;
        self.g_is_6_key_pressed = false;
        self.g_is_7_key_pressed = false;
        self.g_is_8_key_pressed = false;
        self.g_is_9_key_pressed = false;
        self.g_is_0_key_pressed = false;
        self.g_is_f1_key_pressed = false;
        self.g_is_f2_key_pressed = false;
        self.g_is_f3_key_pressed = false;
        self.g_is_f4_key_pressed = false;
        self.g_is_f5_key_pressed = false;
        self.g_is_f6_key_pressed = false;
        self.g_is_f7_key_pressed = false;
        self.g_is_f8_key_pressed = false;
        self.g_is_f9_key_pressed = false;
        self.g_is_f10_key_pressed = false;
        self.g_is_f11_key_pressed = false;
        self.g_is_f12_key_pressed = false;
        self.g_is_minus_key_pressed = false;
        self.g_is_equals_key_pressed = false;
        self.g_is_escape_key_pressed = false;
        self.g_is_space_key_pressed = false;
        self.g_is_enter_key_pressed = false;
        self.g_is_left_control_key_pressed = false;
        self.g_is_left_alt_pressed = false;
        self.g_is_left_shift_pressed = false;
        self.g_is_right_shift_pressed = false;
        self.g_key_pressed = None;
    }

    pub fn update_keyboard_state(&mut self, keys: PressedScancodeIterator) {
        self.reset_pressed_keys();

        for key in keys {
            if self.g_key_pressed.is_none() {
                self.g_key_pressed = Some(key);
            }
            match key {
                Scancode::Escape => self.g_is_escape_key_pressed = true,
                Scancode::Space => self.g_is_space_key_pressed = true,
                Scancode::Left | Scancode::Kp4 => self.g_is_left_key_pressed = true,
                Scancode::Right | Scancode::Kp6 => self.g_is_right_key_pressed = true,
                Scancode::Insert | Scancode::Kp0 => self.g_is_insert_key_pressed = true,
                Scancode::End | Scancode::Kp1 => self.g_is_end_key_pressed = true,
                Scancode::PageUp | Scancode::Kp9 => self.g_is_re_page_key_pressed = true,
                Scancode::PageDown | Scancode::Kp3 => self.g_is_av_page_key_pressed = true,
                Scancode::Kp5 => self.g_is_numpad_5_key_pressed = true,
                Scancode::Home | Scancode::Kp7 => self.g_is_home_key_pressed = true,
                Scancode::Delete | Scancode::KpPeriod => self.g_is_del_key_pressed = true,
                Scancode::KpDivide => self.g_is_numpad_divide_pressed = true,
                Scancode::KpMultiply => self.g_is_numpad_multiply_pressed = true,
                Scancode::KpPlus => self.g_is_numpad_plus_pressed = true,
                Scancode::KpMinus => self.g_is_numpad_minus_pressed = true,
                Scancode::NumLockClear => self.g_is_num_lock_pressed = true,
                Scancode::ScrollLock => self.g_is_scroll_lock_pressed = true,
                Scancode::LAlt => self.g_is_left_alt_pressed = true,
                Scancode::LShift => self.g_is_left_shift_pressed = true,
                Scancode::RShift => self.g_is_right_shift_pressed = true,
                Scancode::LCtrl => self.g_is_left_control_key_pressed = true,
                Scancode::Return => self.g_is_enter_key_pressed = true,
                Scancode::Num1 => self.g_is_1_key_pressed = true,
                Scancode::Num2 => self.g_is_2_key_pressed = true,
                Scancode::Num3 => self.g_is_3_key_pressed = true,
                Scancode::Num4 => self.g_is_4_key_pressed = true,
                Scancode::Num5 => self.g_is_5_key_pressed = true,
                Scancode::Num6 => self.g_is_6_key_pressed = true,
                Scancode::Num7 => self.g_is_7_key_pressed = true,
                Scancode::Num8 => self.g_is_8_key_pressed = true,
                Scancode::Num9 => self.g_is_9_key_pressed = true,
                Scancode::Num0 => self.g_is_0_key_pressed = true,
                Scancode::Minus => self.g_is_minus_key_pressed = true,
                Scancode::Equals => self.g_is_equals_key_pressed = true,
                Scancode::F1 => self.g_is_f1_key_pressed = true,
                Scancode::F2 => self.g_is_f2_key_pressed = true,
                Scancode::F3 => self.g_is_f3_key_pressed = true,
                Scancode::F4 => self.g_is_f4_key_pressed = true,
                Scancode::F5 => self.g_is_f5_key_pressed = true,
                Scancode::F6 => self.g_is_f6_key_pressed = true,
                Scancode::F7 => self.g_is_f7_key_pressed = true,
                Scancode::F8 => self.g_is_f8_key_pressed = true,
                Scancode::F9 => self.g_is_f9_key_pressed = true,
                Scancode::F10 => self.g_is_f10_key_pressed = true,
                Scancode::F11 => self.g_is_f11_key_pressed = true,
                Scancode::F12 => self.g_is_f12_key_pressed = true,
                Scancode::Q => self.g_is_q_key_pressed = true,
                Scancode::W => self.g_is_w_key_pressed = true,
                Scancode::R => self.g_is_r_key_pressed = true,
                Scancode::P => self.g_is_p_key_pressed = true,
                Scancode::S => self.g_is_s_key_pressed = true,
                Scancode::D => self.g_is_d_key_pressed = true,
                Scancode::H => self.g_is_h_key_pressed = true,
                Scancode::J => self.g_is_j_key_pressed = true,
                Scancode::L => self.g_is_l_key_pressed = true,
                Scancode::Z => self.g_is_z_key_pressed = true,
                Scancode::X => self.g_is_x_key_pressed = true,
                Scancode::C => self.g_is_c_key_pressed = true,
                Scancode::B => self.g_is_b_key_pressed = true,
                Scancode::M => self.g_is_m_key_pressed = true,
                _ => (),
            }
        }
    }

    pub fn is_any_key_pressed(&mut self) -> bool {
        self.g_key_pressed.is_some()
    }

    pub fn character_for_last_key_pressed(&mut self) -> char {
        match self.g_key_pressed {
            Some(Scancode::Num0) => '0',
            Some(Scancode::Num1) => '1',
            Some(Scancode::Num2) => '2',
            Some(Scancode::Num3) => '3',
            Some(Scancode::Num4) => '4',
            Some(Scancode::Num5) => '5',
            Some(Scancode::Num6) => '6',
            Some(Scancode::Num7) => '7',
            Some(Scancode::Num8) => '8',
            Some(Scancode::Num9) => '9',
            Some(Scancode::A) => 'A',
            Some(Scancode::B) => 'B',
            Some(Scancode::C) => 'C',
            Some(Scancode::D) => 'D',
            Some(Scancode::E) => 'E',
            Some(Scancode::F) => 'F',
            Some(Scancode::G) => 'G',
            Some(Scancode::H) => 'H',
            Some(Scancode::I) => 'I',
            Some(Scancode::J) => 'J',
            Some(Scancode::K) => 'K',
            Some(Scancode::L) => 'L',
            Some(Scancode::M) => 'M',
            Some(Scancode::N) => 'N',
            Some(Scancode::O) => 'O',
            Some(Scancode::P) => 'P',
            Some(Scancode::Q) => 'Q',
            Some(Scancode::R) => 'R',
            Some(Scancode::S) => 'S',
            Some(Scancode::T) => 'T',
            Some(Scancode::U) => 'U',
            Some(Scancode::V) => 'V',
            Some(Scancode::W) => 'W',
            Some(Scancode::X) => 'X',
            Some(Scancode::Y) => 'Y',
            Some(Scancode::Z) => 'Z',
            Some(Scancode::Space) | Some(Scancode::KpSpace) => ' ',
            Some(Scancode::Minus) | Some(Scancode::KpMinus) => '-',
            Some(Scancode::Return) | Some(Scancode::Return2) | Some(Scancode::KpEnter) => '\n',
            Some(Scancode::Backspace) => '%',
            _ => '\0',
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum UserInput {
    #[default]
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

impl AddAssign<UserInput> for UserInput {
    fn add_assign(&mut self, other: Self) {
        let value = *self as u8 + other as u8;
        if value == 0 {
            *self = UserInput::UserInputNone;
        } else if value == 1 {
            *self = UserInput::UserInputUp;
        } else if value == 2 {
            *self = UserInput::UserInputLeft;
        } else if value == 3 {
            *self = UserInput::UserInputDown;
        } else if value == 4 {
            *self = UserInput::UserInputRight;
        } else if value == 5 {
            *self = UserInput::UserInputSpaceUp;
        } else if value == 6 {
            *self = UserInput::UserInputSpaceLeft;
        } else if value == 7 {
            *self = UserInput::UserInputSpaceDown;
        } else if value == 8 {
            *self = UserInput::UserInputSpaceRight;
        } else if value == 0 {
            *self = UserInput::UserInputSpaceOnly;
        }
    }
}

pub const K_USER_INPUT_SPACE_AND_DIRECTION_OFFSET: UserInput = UserInput::UserInputRight;
