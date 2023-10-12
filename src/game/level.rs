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

use crate::game::globals::*;

#[derive(Clone)]
pub struct Level {
    pub tiles: [u8; K_LEVEL_SIZE], // [0-0x59F] of LevelTileType
    pub unused: [u8; 4],
    pub initial_gravitation: u8,
    pub speed_fix_magic_number: u8, // Used from versions 5.3 and up as: 20h + SpeedFix version number in hex format: v5.3 -> 73h, v6.2 -> 82h
    pub name: String,
    pub freeze_zonks: u8,            // 2 = on, anything else (including 1) = off
    pub number_of_infotrons: u8, // 0 means that Supaplex will count the total amount of Infotrons in the level, and use the low byte of that number. (A multiple of 256 Infotrons will then result in 0-to-eat, etc.!)
    pub number_of_special_ports: u8, // maximum 10
    pub special_ports_info: [SpecialPortInfo; K_LEVEL_MAX_NUMBER_OF_SPECIAL_PORTS],

    // This byte carries the information of the slowest speed used during the demo recording. 0x00=fastest, 0x0A=slowest
    // This information is exclusive-ored with the high random number byte (byte scrambledSpeed). (Each bit is toggled, where in byte highByteRandomSeed a 1 appears.)
    // The result is the value of byte scrambledSpeed (and is used to scramble byte scrambledChecksum).
    pub scrambled_speed: u8,

    // All upper nibbles of each demo byte (without first level number byte and without ending 0xFF), each nibble
    // incremented by 1, are added up. This total equals the total number of demo frames and reflects the normalized
    // demo time with 35 frames per second.
    // To this total, of which only the lower 8 bits are used, the lower random number byte (byte lowByteRandomSeed) is added.
    // The resulting lower 8 bits are exclusive-ored with the final contents of byte scrambledSpeed. (Each bit is toggled,
    // where in byte scrambledSpeed a 1 appears.)
    // The resulting lower 8 bits is the value of byte scrambledChecksum.
    // Note: Megaplex does not put any information into bytes scrambledSpeed and scrambledChecksum.
    pub scrambled_checksum: u8,

    // All Bugs are fired randomly, so in order to be able to make a recording of a level with Bugs, it is necessary to let
    // them fire exactly at the same time in each playback of that recording. In order to guarantee that, we need a
    // predictable random number generator and start it each playback with the same starting value (seed) as when the
    // recording was started. When the sequence of all following random numbers is repeatable, all Bugs will always fire
    // the same way during each playback as during the creation of the recording.
    // Luckily the original Supaplex uses a very simple random number generator for this purpose, which is not depending
    // on external influences like date and time or a keypress. Start the random number generator with a random number
    // seed and the next random number is calculated, which is also used as seed for the next calculation. A certain
    // seed will always result in only one specific random number. The sequence of all following random numbers is
    // thus fixed for each seed.
    // So at the start of each recording, we need to remember the starting random number as seed for the random number
    // generator during each playback.
    // Each random number is a 16 bit number. After each random number calculation, only the lower 16 bits are kept as
    // seed for the next calculation: new_random_number_seed = ((old_random_number_seed * 1509) + 49) modulo 65536
    // This "modulo 65536" just signifies keeping only the lower 16 bits and reject all higher bits.
    pub random_seed: u16,
} // size 1536 = 0x600

impl Level {
    pub fn new() -> Level {
        Level {
            freeze_zonks: 0,
            initial_gravitation: 0,
            number_of_infotrons: 0,
            number_of_special_ports: 0,
            scrambled_checksum: 0,
            scrambled_speed: 0,
            speed_fix_magic_number: 0,
            name: String::new(),
            random_seed: 0,
            special_ports_info: [SpecialPortInfo::default(); K_LEVEL_MAX_NUMBER_OF_SPECIAL_PORTS],
            tiles: [(); K_LEVEL_SIZE].map(|_| 0),
            unused: [0; 4],
        }
    }

    pub fn from_raw(level_index: usize, raw_data: [u8; K_LEVEL_DATA_LENGTH]) -> Level {
        // TODO : fix type and data size issue
        let mut level = Level::new();

        const OFFSET_TILES: usize = 0;
        const OFFSET_UNUSED: usize = OFFSET_TILES + K_LEVEL_SIZE;
        const OFFSET_INITIAL_GRAVITATION: usize = OFFSET_UNUSED + 4;
        const OFFSET_SPEED_FIX_MAGIC_NUMBER: usize = OFFSET_INITIAL_GRAVITATION + 1;
        const OFFSET_LEVEL_NAME: usize = OFFSET_SPEED_FIX_MAGIC_NUMBER + 1;
        const OFFSET_FREEZE_ZONKS: usize = OFFSET_LEVEL_NAME + K_LEVEL_NAME_LENGTH - 1;
        const OFFSET_NUMBER_OF_INFOTRONS: usize = OFFSET_FREEZE_ZONKS + 1;
        const OFFSET_NUMBER_OF_SP: usize = OFFSET_NUMBER_OF_INFOTRONS + 1;
        const OFFSET_SP_0: usize = OFFSET_NUMBER_OF_SP + 1;
        const OFFSET_SP_1: usize = OFFSET_SP_0 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_2: usize = OFFSET_SP_1 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_3: usize = OFFSET_SP_2 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_4: usize = OFFSET_SP_3 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_5: usize = OFFSET_SP_4 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_6: usize = OFFSET_SP_5 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_7: usize = OFFSET_SP_6 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_8: usize = OFFSET_SP_7 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SP_9: usize = OFFSET_SP_8 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SCRAMBLED_CHECKSUM: usize = OFFSET_SP_9 + K_SPECIAL_PORT_STRUCT_SIZE;
        const OFFSET_SCRAMBLED_SPEED: usize = OFFSET_SCRAMBLED_CHECKSUM + 1;
        const OFFSET_RANDOM_SEED: usize = OFFSET_SCRAMBLED_SPEED;

        level.freeze_zonks = raw_data[OFFSET_FREEZE_ZONKS];
        level.initial_gravitation = raw_data[OFFSET_INITIAL_GRAVITATION];
        level.number_of_infotrons = raw_data[OFFSET_NUMBER_OF_INFOTRONS];
        level.number_of_special_ports = raw_data[OFFSET_NUMBER_OF_SP];
        level.scrambled_checksum = raw_data[OFFSET_SCRAMBLED_CHECKSUM];
        level.scrambled_speed = raw_data[OFFSET_SCRAMBLED_SPEED];
        level.speed_fix_magic_number = raw_data[OFFSET_SPEED_FIX_MAGIC_NUMBER];
        level.name = format!(
            "{:03} {}",
            level_index + 1,
            String::from_utf8_lossy(
                &raw_data[OFFSET_LEVEL_NAME..(OFFSET_LEVEL_NAME + K_LEVEL_NAME_LENGTH - 1)]
            )
        );
        level.random_seed =
            (raw_data[OFFSET_RANDOM_SEED] as u16) << 8 | raw_data[OFFSET_RANDOM_SEED + 1] as u16; // LE or BE ?
        level.special_ports_info[0] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_0..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_0)],
        );
        level.special_ports_info[1] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_1..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_1)],
        );
        level.special_ports_info[2] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_2..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_2)],
        );
        level.special_ports_info[3] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_3..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_3)],
        );
        level.special_ports_info[4] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_4..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_4)],
        );
        level.special_ports_info[5] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_5..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_5)],
        );
        level.special_ports_info[6] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_6..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_6)],
        );
        level.special_ports_info[7] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_7..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_7)],
        );
        level.special_ports_info[8] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_8..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_8)],
        );
        level.special_ports_info[9] = SpecialPortInfo::from_raw(
            &raw_data[OFFSET_SP_9..(K_SPECIAL_PORT_STRUCT_SIZE + OFFSET_SP_9)],
        );

        for i in 0..K_LEVEL_SIZE {
            level.tiles[i] = raw_data[i];
        }

        for i in 0..4 {
            level.unused[i] = raw_data[OFFSET_UNUSED + i];
        }

        level
    }
}
