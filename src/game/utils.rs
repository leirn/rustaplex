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

pub fn swap16(value: u16) -> u16 {
    // SDL_Swap16(value) not found in crate interface
    value.swap_bytes()
}

pub fn convert_16le(value: u16) -> u16 {
    // SDL_SwapLE16(value) not found in crate interface
    value.swap_bytes()
}

pub fn clamp<T: Ord>(v: T, a: T, b: T) -> T {
    std::cmp::min(std::cmp::max(a, v), b)
}
