#[derive(Default)]
pub struct ButtonStatus {
    pub gPlayerListButtonPressed: bool,
    pub gPlayerListUpButtonPressed: bool,
    pub gPlayerListDownButtonPressed: bool,
    pub gRankingListButtonPressed: bool,
    pub gRankingListUpButtonPressed: bool,
    pub gRankingListDownButtonPressed: bool,
    pub gLevelListButtonPressed: bool,
    pub gLevelListUpButtonPressed: bool,
    pub gLevelListDownButtonPressed: bool,
}
#[derive(PartialEq)]
pub enum ButtonBorderLineType {
    ButtonBorderLineTypeHorizontal, // from left to right
    ButtonBorderLineTypeVertical,   // from bottom to top
    ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
    ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
}

pub struct ButtonBorderLineDescriptor {
    pub button_type: ButtonBorderLineType,
    pub x: u16,
    pub y: u16,
    pub length: u16,
}

const K_NUMBER_OF_OPTIONS_MENU_BORDERS: usize = 20;

const K_NUMBER_OF_MAIN_MENU_BORDERS: usize = 12;

pub const K_OPTIONS_MENU_BORDERS: [&[ButtonBorderLineDescriptor];
    K_NUMBER_OF_OPTIONS_MENU_BORDERS] = [
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 144,
            y: 81,
            length: 5,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 127,
            y: 76,
            length: 18,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 122,
            y: 71,
            length: 5,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 122,
            y: 70,
            length: 69,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 76,
            y: 2,
            length: 46,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 70,
            y: 8,
            length: 7,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 103,
            y: 39,
            length: 16,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 118,
            y: 77,
            length: 38,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 119,
            y: 78,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 124,
            y: 85,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 124,
            y: 112,
            length: 8,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 121,
            y: 115,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 120,
            y: 117,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 121,
            y: 126,
            length: 24,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 144,
            y: 127,
            length: 2,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 94,
            y: 44,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 94,
            y: 42,
            length: 19,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 113,
            y: 81,
            length: 40,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 114,
            y: 82,
            length: 14,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 94,
            y: 78,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 95,
            y: 78,
            length: 11,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 106,
            y: 79,
            length: 9,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 115,
            y: 129,
            length: 42,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 116,
            y: 129,
            length: 20,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 88,
            y: 115,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 89,
            y: 115,
            length: 23,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 112,
            y: 115,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 119,
            y: 94,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 120,
            y: 93,
            length: 8,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 120,
            y: 94,
            length: 8,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 22,
            y: 114,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 11,
            y: 114,
            length: 11,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 10,
            y: 151,
            length: 38,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 6,
            y: 151,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 5,
            y: 156,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 14,
            y: 155,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 15,
            y: 153,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 21,
            y: 160,
            length: 8,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 22,
            y: 160,
            length: 20,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 42,
            y: 159,
            length: 9,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 51,
            y: 151,
            length: 74,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 125,
            y: 152,
            length: 10,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 135,
            y: 161,
            length: 9,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 144,
            y: 161,
            length: 5,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 34,
            y: 113,
            length: 5,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 70,
            y: 116,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 73,
            y: 113,
            length: 5,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 73,
            y: 84,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 72,
            y: 76,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 9,
            y: 111,
            length: 5,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 9,
            y: 112,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 7,
            y: 112,
            length: 68,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 8,
            y: 112,
            length: 68,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 9,
            y: 45,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 9,
            y: 46,
            length: 5,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 138,
            y: 37,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 125,
            y: 37,
            length: 13,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 124,
            y: 67,
            length: 31,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 125,
            y: 68,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 131,
            y: 73,
            length: 19,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 150,
            y: 81,
            length: 9,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 151,
            y: 36,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 182,
            y: 39,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 192,
            y: 38,
            length: 3,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 156,
            y: 65,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 156,
            y: 69,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 157,
            y: 69,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 182,
            y: 71,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 183,
            y: 85,
            length: 14,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 183,
            y: 102,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 183,
            y: 124,
            length: 13,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 168,
            y: 139,
            length: 15,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 165,
            y: 139,
            length: 3,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 180,
            y: 65,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 180,
            y: 69,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 181,
            y: 69,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 182,
            y: 71,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 183,
            y: 85,
            length: 14,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 183,
            y: 102,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 183,
            y: 124,
            length: 13,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 168,
            y: 139,
            length: 15,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 165,
            y: 139,
            length: 3,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 180,
            y: 90,
            length: 6,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 186,
            y: 90,
            length: 24,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 187,
            y: 67,
            length: 38,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 225,
            y: 67,
            length: 33,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 226,
            y: 35,
            length: 33,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 150,
            y: 160,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 150,
            y: 161,
            length: 31,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 181,
            y: 161,
            length: 29,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 181,
            y: 133,
            length: 8,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 189,
            y: 125,
            length: 22,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 189,
            y: 103,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 192,
            y: 99,
            length: 25,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 192,
            y: 74,
            length: 51,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal,
            x: 243,
            y: 74,
            length: 17,
        },
    ],
    &[ButtonBorderLineDescriptor {
        button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
        x: 257,
        y: 126,
        length: 10,
    }],
    &[ButtonBorderLineDescriptor {
        button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
        x: 257,
        y: 131,
        length: 10,
    }],
    &[ButtonBorderLineDescriptor {
        button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
        x: 257,
        y: 136,
        length: 10,
    }],
    &[ButtonBorderLineDescriptor {
        button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
        x: 257,
        y: 141,
        length: 10,
    }],
    &[ButtonBorderLineDescriptor {
        button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
        x: 257,
        y: 146,
        length: 10,
    }],
    &[ButtonBorderLineDescriptor {
        button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
        x: 257,
        y: 151,
        length: 10,
    }],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 257,
            y: 116,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 261,
            y: 119,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 257,
            y: 111,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 259,
            y: 112,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 268,
            y: 113,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 272,
            y: 114,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 273,
            y: 114,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 280,
            y: 113,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 280,
            y: 114,
            length: 7,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 283,
            y: 119,
            length: 3,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 292,
            y: 119,
            length: 2,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 293,
            y: 131,
            length: 12,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 289,
            y: 131,
            length: 4,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal,
            x: 283,
            y: 126,
            length: 4,
        },
    ],
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 159,
            y: 180,
            length: 14,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 160,
            y: 167,
            length: 28,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 187,
            y: 166,
            length: 8,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 187,
            y: 149,
            length: 10,
        },
    ],
];

pub const K_MAIN_MENU_BUTTON_BORDERS: [&[ButtonBorderLineDescriptor];
    K_NUMBER_OF_MAIN_MENU_BORDERS] = [
    // starts on 0x504? or before?
    // Player List - Up Arrow - Left and Top borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 11,
            y: 152,
            length: 11,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 12,
            y: 142,
            length: 56,
        },
    ],
    // Player List - Up Arrow - Bottom and Right borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 12,
            y: 153,
            length: 56,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 67,
            y: 153,
            length: 11,
        },
    ],
    // Player List - Down Arrow - Left and Top borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 11,
            y: 191,
            length: 11,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 12,
            y: 181,
            length: 56,
        },
    ],
    // Player List - Down Arrow - Bottom and Right borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 12,
            y: 192,
            length: 56,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 67,
            y: 192,
            length: 11,
        },
    ],
    // Ranking List - Up Arrow - Left and Top borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 141,
            y: 105,
            length: 16,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 142,
            y: 90,
            length: 13,
        },
    ],
    // Ranking List - Up Arrow - Bottom and Right borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 141,
            y: 106,
            length: 14,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 154,
            y: 105,
            length: 15,
        },
    ],
    // Ranking List - Down Arrow - Left and Top borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 141,
            y: 135,
            length: 16,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 142,
            y: 120,
            length: 13,
        },
    ],
    // Ranking List - Down Arrow - Bottom and Right borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 141,
            y: 136,
            length: 14,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 154,
            y: 135,
            length: 15,
        },
    ],
    // Level List - Up Arrow - Left and Top borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 142,
            y: 152,
            length: 11,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 143,
            y: 142,
            length: 163,
        },
    ],
    // Level List - Up Arrow - Bottom and Right borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 143,
            y: 153,
            length: 164,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 306,
            y: 152,
            length: 11,
        },
    ],
    // Level List - Down Arrow - Left and Top borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 142,
            y: 191,
            length: 11,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 143,
            y: 181,
            length: 164,
        },
    ],
    // Level List - Down Arrow - Bottom and Right borders
    &[
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeHorizontal,
            x: 142,
            y: 192,
            length: 164,
        },
        ButtonBorderLineDescriptor {
            button_type: ButtonBorderLineType::ButtonBorderLineTypeVertical,
            x: 306,
            y: 192,
            length: 12,
        },
    ],
];

use crate::Game;

pub struct ButtonDescriptor {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub callback: fn(&mut Game<'_>),
}

pub const K_NUMBER_OF_MAIN_MENU_BUTTONS: usize = 17;

pub const K_MAIN_MENU_BUTTON_DESCRIPTORS: [ButtonDescriptor; K_NUMBER_OF_MAIN_MENU_BUTTONS] = [
    ButtonDescriptor {
        start_x: 5,
        start_y: 6,
        end_x: 157,
        end_y: 14,
        callback: |game| game.handleNewPlayerOptionClick(), // New player
    },
    ButtonDescriptor {
        start_x: 5,
        start_y: 15,
        end_x: 157,
        end_y: 23,
        callback: |game| game.handleDeletePlayerOptionClick(), // Delete player
    },
    ButtonDescriptor {
        start_x: 5,
        start_y: 24,
        end_x: 157,
        end_y: 32,
        callback: |game| game.handleSkipLevelOptionClick(), // Skip level
    },
    ButtonDescriptor {
        start_x: 5,
        start_y: 33,
        end_x: 157,
        end_y: 41,
        callback: |game| game.handleStatisticsOptionClick(), // Statistics
    },
    ButtonDescriptor {
        start_x: 5,
        start_y: 42,
        end_x: 157,
        end_y: 50,
        callback: |game| game.handleGfxTutorOptionClick(), // GFX-tutor
    },
    ButtonDescriptor {
        start_x: 5,
        start_y: 51,
        end_x: 157,
        end_y: 59,
        callback: |game| game.handleDemoOptionClick(), // Demo
    },
    ButtonDescriptor {
        start_x: 5,
        start_y: 60,
        end_x: 157,
        end_y: 69,
        callback: |game| game.handleControlsOptionClick(), // Controls
    },
    ButtonDescriptor {
        start_x: 140,
        start_y: 90,
        end_x: 155,
        end_y: 108,
        callback: |game| game.handleRankingListScrollUp(), // Rankings arrow up
    },
    ButtonDescriptor {
        start_x: 140,
        start_y: 121,
        end_x: 155,
        end_y: 138,
        callback: |game| game.handleRankingListScrollDown(), // Rankings arrow down
    },
    ButtonDescriptor {
        start_x: 96,
        start_y: 140,
        end_x: 115,
        end_y: 163,
        callback: |game| game.handleOkButtonClick(), // Ok button
    },
    ButtonDescriptor {
        start_x: 83,
        start_y: 168,
        end_x: 126,
        end_y: 192,
        callback: |game| game.handleFloppyDiskButtonClick(), // Insert data disk according to https://supaplex.fandom.com/wiki/Main_menu
    },
    ButtonDescriptor {
        start_x: 11,
        start_y: 142,
        end_x: 67,
        end_y: 153,
        callback: |game| game.handlePlayerListScrollUp(), // Players arrow up
    },
    ButtonDescriptor {
        start_x: 11,
        start_y: 181,
        end_x: 67,
        end_y: 192,
        callback: |game| game.handlePlayerListScrollDown(), // Players arrow down
    },
    ButtonDescriptor {
        start_x: 11,
        start_y: 154,
        end_x: 67,
        end_y: 180,
        callback: |game| game.handlePlayerListClick(), // Players list area
    },
    ButtonDescriptor {
        start_x: 142,
        start_y: 142,
        end_x: 306,
        end_y: 153,
        callback: |game| game.handleLevelListScrollUp(), // Levels arrow up
    },
    ButtonDescriptor {
        start_x: 142,
        start_y: 181,
        end_x: 306,
        end_y: 192,
        callback: |game| game.handleLevelListScrollDown(), // Levels arrow down
    },
    ButtonDescriptor {
        start_x: 297,
        start_y: 37,
        end_x: 312,
        end_y: 52,
        callback: |game| game.handleLevelCreditsClick(), // Credits
    },
];
