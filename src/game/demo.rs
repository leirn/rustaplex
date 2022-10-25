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

pub const K_NUMBER_OF_DEMOS: usize = 10;

use crate::game::globals::*;
use crate::game::level::Level;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub struct DemoManager {
    pub g_levels_dat_filename: String,
    pub g_sp_demo_filename: String,
    pub g_is_sp_demo_available_to_run: u8,
    pub file_is_demo: bool,
    pub g_is_playing_demo: bool,
    pub g_demo_current_input_index: u16,

    k_original_demo_file_sizes: [u16; K_NUMBER_OF_DEMOS],
    k_original_demo_first_file_chunks: [FirstOriginalDemoFileChunk; K_NUMBER_OF_DEMOS],

    g_demo_random_seeds: [u16; K_NUMBER_OF_DEMOS],

    g_demos: Demos,

    g_current_demo_level_name: String,

    recording_demo_message: String,

    g_selected_original_demo_level_number: usize,
    demo_file_name: String,
    g_demo0_bin_filename: String,
}
impl DemoManager {
    pub fn new() -> DemoManager {
        DemoManager {
            g_levels_dat_filename: String::from("LEVELS.DAT"),
            g_sp_demo_filename: String::from("00S001-0.SP"),
            g_is_sp_demo_available_to_run: 0,
            file_is_demo: false,
            g_is_playing_demo: false,
            g_demo_current_input_index: 0,
            g_current_demo_level_name: String::from(".SP\0----- DEMO LEVEL! -----"),
            g_demo_random_seeds: [0; K_NUMBER_OF_DEMOS],
            g_demos: Demos::new(),
            k_original_demo_file_sizes: [
                0x00cE, 0x016a, 0x0146, 0x00cd, 0x024d, 0x012c, 0x01a7, 0x01fb, 0x01d2, 0x02fd,
            ],
            k_original_demo_first_file_chunks: [
                FirstOriginalDemoFileChunk {
                    level_number: 0x01,
                    first_user_inputs: [0xf0, 0xf0, 0xf1],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x03,
                    first_user_inputs: [0xf0, 0x50, 0xf3],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x07,
                    first_user_inputs: [0xf0, 0x60, 0xf4],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x0B,
                    first_user_inputs: [0xf0, 0xf0, 0xf0],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x1D,
                    first_user_inputs: [0xf0, 0xf0, 0xf0],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x26,
                    first_user_inputs: [0xf0, 0xf0, 0x50],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x37,
                    first_user_inputs: [0xf0, 0xd0, 0x41],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x5F,
                    first_user_inputs: [0x10, 0xf3, 0xf3],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x68,
                    first_user_inputs: [0xf0, 0xf0, 0x10],
                },
                FirstOriginalDemoFileChunk {
                    level_number: 0x6C,
                    first_user_inputs: [0x10, 0xf4, 0x94],
                },
            ],
            recording_demo_message: String::from("--- RECORDING DEMO0 ---"),
            g_selected_original_demo_level_number: 0,
            demo_file_name: String::new(),
            g_demo0_bin_filename: String::from("DEMO0.BIN"),
        }
    }

    pub fn get_level_number_from_original_demo_file(&self, file: File, fileLength: u64) -> usize {
        0
    }

    pub fn prepare_demo_recording_filename(&mut self) {
        if self.g_levels_dat_filename[8..10] == String::from("00") {
            self.g_sp_demo_filename.replace_range(2..4, "--");
        } else if self.g_levels_dat_filename[8..10] == String::from("AT") {
            self.g_sp_demo_filename.replace_range(2..4, "00");
        }
    }

    pub fn read_demo_files(&mut self) -> u8 {
        self.g_demo_current_input_index = 0;

        self.g_demos.demo_first_indices = [0xffff; K_NUMBER_OF_DEMOS + 1];
        //word_5A33C = 0;

        for i in 0..K_NUMBER_OF_DEMOS {
            self.g_selected_original_demo_level_number = 0;
            let filename = self.g_demo0_bin_filename.clone();
            let mut filename = filename.as_str();

            if self.g_is_sp_demo_available_to_run == 1 {
                filename = self.demo_file_name.as_str();
            } else {
                let value = ('0' as u8 + i as u8) as char;
                self.g_demo0_bin_filename
                    .replace_range(4..5, String::from(value).as_str());
            }

            let file_path = Path::new(filename);
            match file_path
                .try_exists()
                .expect(format!("Can't check existence of file {}", filename).as_str())
            {
                true => (),
                false => return i as u8,
            }
            let mut file =
                File::open(file_path).expect(format!("Error while opening {}", filename).as_str());

            if self.g_is_sp_demo_available_to_run == 1 {
                // Select demo from command line not implemented
                //if (gSelectedOriginalDemoFromCommandLineLevelNumber == 0)
                //        {
                file.seek(SeekFrom::Start(K_LEVEL_DATA_LENGTH as u64))
                    .unwrap();
                //fseek(file, K_LEVEL_DATA_LENGTH, SEEK_SET);
                // }
            } else {
                file.seek(SeekFrom::End(0)).unwrap();
                let file_size = file.stream_position().unwrap();
                if file_size < K_LEVEL_DATA_LENGTH as u64 {
                    // TODO : issue to solve : self.g_selected_original_demo_level_number = self.get_level_number_from_original_demo_file(file, file_size);
                }
                file.seek(SeekFrom::Start(0)).unwrap();

                if self.g_selected_original_demo_level_number == 0 {
                    let mut level_buffer = [0_u8; K_LEVEL_DATA_LENGTH];
                    let bytes = file.read(&mut level_buffer).unwrap();

                    if bytes < K_LEVEL_DATA_LENGTH {
                        return i as u8;
                    }
                    let level = Level::from_raw(level_buffer);
                    self.g_demos.level[i] = level.clone();
                    self.g_demo_random_seeds[i] = level.random_seed;
                }
            }
            let mut max_number_of_bytes_to_read = K_MAX_DEMO_INPUT_STEPS as u16 + 1;
            max_number_of_bytes_to_read -= self.g_demo_current_input_index;
            if max_number_of_bytes_to_read > K_MAX_DEMO_INPUT_STEPS as u16 + 1 {
                max_number_of_bytes_to_read = 0;
            }

            let mut number_of_demo_bytes_read = 0;

            if max_number_of_bytes_to_read == 0 {
                number_of_demo_bytes_read = 0;
            } else {
                let mut data_buffer: Vec<u8> = vec![0; max_number_of_bytes_to_read as usize];
                number_of_demo_bytes_read = file.read(&mut data_buffer).unwrap();

                self.g_demos.demo_data[(self.g_demo_current_input_index as usize)..].copy_from_slice(data_buffer.as_slice());

                if (number_of_demo_bytes_read == 0) {
                    return i as u8;
                }
            }
            self.g_demos.demo_data[self.g_demo_current_input_index as usize] = self.g_demos.demo_data[self.g_demo_current_input_index as usize] & 0x7f; // this removes the MSB from the levelNumber that was added in the speed fix mods
            let is_zero = self.g_selected_original_demo_level_number == 0;
            self.g_selected_original_demo_level_number = 0;
            if is_zero
            {
                self.g_demos.demo_data[self.g_demo_current_input_index as usize] = self.g_demos.demo_data[self.g_demo_current_input_index as usize] | 0x80; // This sets the MSB?? maybe the "interpreter" later needs it
            }

            let demo_last_byte_index = self.g_demo_current_input_index as usize + number_of_demo_bytes_read - 1;
            if demo_last_byte_index == 0xffff // this would mean bx was 0. is this possible?
                || number_of_demo_bytes_read <= 1 // this means the demo is empty (only has levelNumber or nothing)
                || self.g_demos.demo_data[demo_last_byte_index] != 0xff
            {
                if demo_last_byte_index < K_BASE_DEMO_SIZE
                {
                    number_of_demo_bytes_read += 1;
                    self.g_demos.demo_data[demo_last_byte_index + 1] = 0xff;
                }
            }

            self.g_demos.demo_first_indices[i] = self.g_demo_current_input_index;
            self.g_demo_current_input_index += number_of_demo_bytes_read as u16;
        }

        K_NUMBER_OF_DEMOS as u8
    }
}

// These are literally the first 4 bytes of the original files, used by the spfix version to detect when a demo from
// the original game was being parsed (since those had a different format).

#[derive(Default, Clone, Copy)]
struct FirstOriginalDemoFileChunk {
    pub level_number: u8,
    pub first_user_inputs: [u8; 3],
}

const K_MAX_DEMO_INPUT_STEPS: usize = 48648;
const K_MAX_BASE_DEMO_SIZE: usize = 1 + K_MAX_DEMO_INPUT_STEPS + 1;

// This struct defines the demo format of the original game (previous to the speed fix mods)
#[derive(Clone, Copy)]
struct BaseDemo {
    // In demos recorded with the original game (not speed fix versions), it was the level number. After the speed fix
    // versions, the level itself was included in the demo file, so they set the MSB to 1 to mark them and this byte
    // loses its original function.
    //
    pub level_number: u8,
    pub input_steps: [u8; K_MAX_DEMO_INPUT_STEPS + 1], // of UserInput, finishes with 0xFF
}

const K_BASE_DEMO_SIZE: usize = 4;

const K_MAX_DEMO_SIGNATURE_LENGTH: usize = 511;
const K_MAX_DEMO_SIGNATURE_SIZE: usize = K_MAX_DEMO_SIGNATURE_LENGTH + 1;

// This struct defines the demo format of the game after the speed fix mods (which includes a demo of the original
// format inside).
#[derive(Clone, Copy)]
struct DemoFile {
    pub level: Level,
    pub base_demo: BaseDemo,
    pub signature: [u8; K_MAX_DEMO_SIGNATURE_SIZE], // text that ends with 0xFF
}

#[derive(Clone, Copy)]
struct Demos {
    pub demo_first_indices: [u16; K_NUMBER_OF_DEMOS + 1], // index of the last byte of all demos (starting at demo-segment:0000). there are 11 words because the end of this "list" is marked with 0xFFFF
    pub demo_data: [u8; 1 + K_MAX_DEMO_INPUT_STEPS + 1], // to fit at least one huge demo with 1 byte for level number, then all the possible steps, then 0xFF
    pub level: [Level; K_NUMBER_OF_DEMOS],
}

impl Demos {
    pub fn new() -> Demos {
        Demos {
            demo_first_indices: [0xff; K_NUMBER_OF_DEMOS + 1],
            demo_data: [(); 1 + K_MAX_DEMO_INPUT_STEPS + 1].map(|_| 0),
            level: [Level::new(); K_NUMBER_OF_DEMOS],
        }
    }
}
