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

    g_current_demo_level_name: [char; K_LIST_LEVEL_NAME_LENGTH],

    recording_demo_message: [char; K_LIST_LEVEL_NAME_LENGTH],

    g_selected_original_demo_level_number : usize,
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
            g_current_demo_level_name: [' '; K_LIST_LEVEL_NAME_LENGTH],
            g_demo_random_seeds: [0; K_NUMBER_OF_DEMOS],
            g_demos: Demos::new(),
            k_original_demo_file_sizes: [0; K_NUMBER_OF_DEMOS],
            k_original_demo_first_file_chunks: [FirstOriginalDemoFileChunk::default();
                K_NUMBER_OF_DEMOS],
            recording_demo_message: [' '; K_LIST_LEVEL_NAME_LENGTH],
            g_selected_original_demo_level_number: 0,
            demo_file_name: String::new(),
            g_demo0_bin_filename: String::from("DEMO0.BIN"),
        }
    }

    pub fn getLevelNumberFromOriginalDemoFile(&self, file: File, fileLength: u16) -> u8 {
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
            let mut filename = self.g_demo0_bin_filename.as_str();

            if self.g_is_sp_demo_available_to_run ==1 {
                filename = self.demo_file_name.as_str();
            }
            else {

                let value = ('0' as u8 + i as u8) as char;
                self.g_demo0_bin_filename.replace_range(4..5, String::from(value).as_str());
            }
            /*


            //loc_47647:             // ; CODE XREF: readDemoFiles+31j
                    FILE *file = openWritableFileWithReadonlyFallback(filename, "rb");
                    if (file == NULL)
                    {
                        return i;
                    }

            //loc_47651:              //; CODE XREF: readDemoFiles+43j
                    if (gIsSPDemoAvailableToRun == 1)
                    {
                        if (gSelectedOriginalDemoFromCommandLineLevelNumber == 0)
                        {
                            fseek(file, kLevelDataLength, SEEK_SET);
                        }
                    }
                    else
                    {
            //loc_47674:             // ; CODE XREF: readDemoFiles+52j
                        int result = fseek(file, 0, SEEK_END);
                        long fileSize = ftell(file);

                        // this is probably to support old level formats
                        if (result == 0
                            && fileSize < kLevelDataLength)
                        {
                            gSelectedOriginalDemoLevelNumber = getLevelNumberFromOriginalDemoFile(file, fileSize);
                        }

            //loc_47690:             // ; CODE XREF: readDemoFiles+76j readDemoFiles+7Aj ...
                        fseek(file, 0, SEEK_SET);

                        if (gSelectedOriginalDemoLevelNumber == 0)
                        {
                            Level *level = &gDemos.level[i];
                            size_t bytes = fileReadBytes(level, kLevelDataLength, file);

                            if (bytes < kLevelDataLength)
                            {
                                return i;
                            }

            //loc_476D3:           //   ; CODE XREF: readDemoFiles+C5j
                            gDemoRandomSeeds[i] = level->randomSeed;
                        }
                    }

            //loc_476DB:             // ; CODE XREF: readDemoFiles+59j readDemoFiles+69j ...
                    uint16_t maxNumberOfBytesToRead = kMaxDemoInputSteps + 1; // 48649
                    maxNumberOfBytesToRead -= gDemoCurrentInputIndex;

                    if (maxNumberOfBytesToRead > kMaxDemoInputSteps + 1) // weird way of checking if gDemoCurrentInputIndex < 0 ????
                    {
                        maxNumberOfBytesToRead = 0;
                    }

                    uint16_t numberOfDemoBytesRead = 0;

            //loc_476EA:             // ; CODE XREF: readDemoFiles+DDj
                    if (maxNumberOfBytesToRead == 0)
                    {
                        numberOfDemoBytesRead = 0;
                    }
                    else
                    {
            //loc_476F3:              // ; CODE XREF: readDemoFiles+E4j
                        numberOfDemoBytesRead = fileReadBytes(&gDemos.demoData[gDemoCurrentInputIndex], maxNumberOfBytesToRead, file);

                        if (numberOfDemoBytesRead == 0)
                        {
                            if (fclose(file) != 0)
                            {
                                exitWithError("Error closing DEMO file");
                            }
                            return i;
                        }

            //loc_47719:             // ; CODE XREF: readDemoFiles+FCj
                    }

            //loc_4771A:             // ; CODE XREF: readDemoFiles+E8j
                    if (fclose(file) != 0)
                    {
                        exitWithError("Error closing DEMO file");
                    }

            //loc_47729:              ; CODE XREF: readDemoFiles+11Bj
                    gDemos.demoData[gDemoCurrentInputIndex] = gDemos.demoData[gDemoCurrentInputIndex] & 0x7F; // this removes the MSB from the levelNumber that was added in the speed fix mods
                    int isZero = (gSelectedOriginalDemoLevelNumber == 0);
                    gSelectedOriginalDemoLevelNumber = 0;
                    if (isZero)
                    {
                        gDemos.demoData[gDemoCurrentInputIndex] = gDemos.demoData[gDemoCurrentInputIndex] | 0x80; // This sets the MSB?? maybe the "interpreter" later needs it
                    }

            //loc_47743:             // ; CODE XREF: readDemoFiles+134j
                    uint16_t demoLastByteIndex = gDemoCurrentInputIndex + numberOfDemoBytesRead - 1;
                    // cx = bx; // bx here has the value of gDemoCurrentInputIndex
                    // bx += numberOfDemoBytesRead; // ax here has the number of bytes read regarding the level itself (levelNumber + inputSteps)
                    // push(ds);
                    // push(es);
                    // pop(ds);
                    // assume ds:nothing
                    // bx--;
                    if (demoLastByteIndex == 0xFFFF // this would mean bx was 0. is this possible?
                        || numberOfDemoBytesRead <= 1 // this means the demo is empty (only has levelNumber or nothing)
                        || gDemos.demoData[demoLastByteIndex] != 0xFF)
                    {
            //loc_4775A:             // ; CODE XREF: readDemoFiles+145j
                       // ; readDemoFiles+14Aj
                        if (demoLastByteIndex < sizeof(BaseDemo))
                        {
                            numberOfDemoBytesRead++;
                            gDemos.demoData[demoLastByteIndex + 1] = 0xFF;
                        }
                    }

            //loc_47765:             // ; CODE XREF: readDemoFiles+14Fj
                               // ; readDemoFiles+155j
                    gDemos.demoFirstIndices[i] = gDemoCurrentInputIndex;
                    gDemoCurrentInputIndex += numberOfDemoBytesRead;

                    */
        }

        K_NUMBER_OF_DEMOS as u8
    }
}

// These are literally the first 4 bytes of the original files, used by the spfix version to detect when a demo from
// the original game was being parsed (since those had a different format).

#[derive(Default, Clone, Copy)]
struct FirstOriginalDemoFileChunk {
    pub levelNumber: u8,
    pub firstUserInputs: [u8; 3],
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
