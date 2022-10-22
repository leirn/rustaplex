pub fn swap16( value:u16) -> u16
{
    // SDL_Swap16(value) not found in crate interface
    value << 8 | value >> 8
}

pub fn convert_16le( value: u16) -> u16
{
    // SDL_SwapLE16(value) not found in crate interface
    value << 8 | value >> 8
}
