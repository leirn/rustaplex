use std::ops::AddAssign;

#[derive(Default, Clone)]
pub struct Keys {
    pub gIsF1KeyPressed: bool,
    pub gIsF2KeyPressed: bool,
    pub gIsF3KeyPressed: bool,
    pub gIsF4KeyPressed: bool,
    pub gIsF5KeyPressed: bool,
    pub gIsF6KeyPressed: bool,
    pub gIsF7KeyPressed: bool,
    pub gIsF8KeyPressed: bool,
    pub gIsF9KeyPressed: bool,
    pub gIsF10KeyPressed: bool,
    pub gIsF12KeyPressed: bool,
    pub gIsNumpadDividePressed: bool,
    pub gIsEscapeKeyPressed: bool,
    pub gCurrentUserInput: UserInput,
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
