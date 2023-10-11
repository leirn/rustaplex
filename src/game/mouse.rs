#[derive(Default)]
pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub button_status: u8,
}

pub const MOUSE_BUTTON_LEFT: u8 = 1;
pub const MOUSE_BUTTON_RIGHT: u8 = 2;
