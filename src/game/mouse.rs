#[derive(Default)]
pub struct Mouse {
    pub g_mouse_x: i32,
    pub g_mouse_y: i32,
    pub g_mouse_button_status: u8,
}

pub const MOUSE_BUTTON_LEFT: u8 = 1;
pub const MOUSE_BUTTON_RIGHT: u8 = 2;
