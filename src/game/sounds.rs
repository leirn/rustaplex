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

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mixer::Chunk;
use sdl2::mixer::{LoaderRWops, Music};
use sdl2::rwops::RWops;

use crate::game::globals::*;

const K_ROLAND_SOUND_FILENAME_SUFFIX: &str = "roland";
const K_ADLIB_SOUND_FILENAME_SUFFIX: &str = "adlib";
const K_BLASTER_SOUND_FILENAME_SUFFIX: &str = "blaster";
const K_SAMPLES_SOUND_FILENAME_SUFFIX: &str = "sample";
const K_STANDARD_SOUND_FILENAME_SUFFIX: &str = "beep";

const G_SOUND_EFFECT_NAMES: [&str; SOUND_EFFECT_COUNT] = [
    "explosion",
    "infotron",
    "push",
    "fall",
    "bug",
    "base",
    "exit",
];
const SOUND_EFFECT_COUNT: usize = 7;

const K_AUDIO_BUFFER_SIZE: u32 = 512;

const K_SAMPLE_RATE: u32 = 44100;
const K_NUMBER_OF_CHANNELS: u32 = 2;

#[derive(PartialEq)]
pub enum SoundType {
    None = 0,
    InternalStandard = 1,
    InternalSamples = 2,
    Adlib = 3,
    SoundBlaster = 4,
    Roland = 5,
}

pub enum SoundEffect {
    Explosion,
    Infotron,
    Push,
    Fall,
    Bug,
    Base,
    Exit,
    Count,
}

pub struct Sounds<'a> {
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    pub is_music_enabled: bool,
    pub is_fx_enabled: bool,
    g_current_sound_priority: u8,
    g_current_sound_duration: u8,
    pub snd_type: SoundType,
    pub mus_type: SoundType,
    g_current_sound_channel: i32,
    g_is_audio_initialized: bool,
    g_music: Option<Music<'a>>,
    g_sound_effect_chunks: [Option<Chunk>; SOUND_EFFECT_COUNT],
}

impl Sounds<'_> {
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Sounds<'static> {
        // https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/mixer-demo.rs
        // https://lib.rs/crates/sdl2_mixer
        Sounds {
            sdl_context: sdl_context,
            is_music_enabled: false,
            is_fx_enabled: false,
            g_current_sound_priority: 0,
            g_current_sound_duration: 0,
            snd_type: SoundType::None,
            mus_type: SoundType::InternalStandard,
            g_current_sound_channel: -1,
            g_is_audio_initialized: false,
            g_music: None,
            g_sound_effect_chunks: [(); SOUND_EFFECT_COUNT].map(|_| None),
        }
    }
}

impl Sounds<'_> {
    pub fn sound_shutdown(&mut self) {
        self.stop_music_and_sounds();
    }

    pub fn activate_internal_standard_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::InternalStandard, SoundType::InternalStandard);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_internal_samples_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::InternalStandard, SoundType::InternalSamples);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_adlib_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::Adlib, SoundType::Adlib);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_sound_blaster_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::Adlib, SoundType::SoundBlaster);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_roland_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::Roland, SoundType::Roland);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_combined_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::Roland, SoundType::SoundBlaster);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn stop_music_and_sounds(&mut self) {
        self.set_sound_type(SoundType::None, SoundType::None);
    }

    pub fn play_music_if_needed(&mut self) {
        if self.is_music_enabled == false {
            return;
        }

        self.play_music();
    }

    pub fn play_explosion_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        if self.g_current_sound_priority >= 5 {
            return;
        }

        self.g_current_sound_priority = 5;
        self.g_current_sound_duration = 0xf;

        self.play_sound_effect(SoundEffect::Explosion);
    }

    pub fn play_infotron_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        if self.g_current_sound_priority >= 5 {
            return;
        }

        self.g_current_sound_priority = 4;
        self.g_current_sound_duration = 0xf;

        self.play_sound_effect(SoundEffect::Infotron);
    }

    pub fn play_push_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        if self.g_current_sound_priority >= 2 {
            return;
        }

        self.g_current_sound_priority = 2;
        self.g_current_sound_duration = 7;

        self.play_sound_effect(SoundEffect::Push);
    }

    pub fn play_fall_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        if self.g_current_sound_priority >= 2 {
            return;
        }

        self.g_current_sound_priority = 2;
        self.g_current_sound_duration = 7;

        self.play_sound_effect(SoundEffect::Fall);
    }

    pub fn play_bug_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        if self.g_current_sound_priority >= 3 {
            return;
        }

        self.g_current_sound_priority = 3;
        self.g_current_sound_duration = 3;

        self.play_sound_effect(SoundEffect::Bug);
    }

    pub fn play_base_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        if self.g_current_sound_priority >= 1 {
            return;
        }

        self.g_current_sound_priority = 1;
        self.g_current_sound_duration = 3;

        self.play_sound_effect(SoundEffect::Base);
    }

    pub fn play_exit_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        self.g_current_sound_priority = 0xa;
        self.g_current_sound_duration = 0xfa;
        self.stop_music();

        self.play_sound_effect(SoundEffect::Exit);
    }

    fn set_sound_type(&mut self, music_type: SoundType, effects_type: SoundType) {
        if self.g_is_audio_initialized == false {
            return;
        }
        self.destroy_music();
        self.destroy_sounds();

        self.snd_type = effects_type;
        self.mus_type = music_type;

        self.load_music();
        self.load_sounds();
    }

    fn play_sound_effect(&mut self, effects_type: SoundEffect) {}

    fn play_music(&mut self) {}

    pub fn stop_music(&mut self) {}

    fn load_music(&mut self) {
        if self.g_is_audio_initialized == false {
            return;
        }

        let music_suffix = match self.mus_type {
            SoundType::Roland => K_ROLAND_SOUND_FILENAME_SUFFIX,
            SoundType::Adlib | SoundType::SoundBlaster => K_BLASTER_SOUND_FILENAME_SUFFIX,
            SoundType::InternalSamples | SoundType::InternalStandard => {
                K_STANDARD_SOUND_FILENAME_SUFFIX
            }
            SoundType::None => return,
        };

        let filename = format!("{}/music-{}.xm", K_BASE_AUDIO_FOLDER, music_suffix);

        let chunk: RWops<'static> = sdl2::rwops::RWops::from_file(filename, "r").unwrap();
        //let chunk: Result<Music<'static>, String> = chunk.load_music();
        //self.g_music = chunk.ok();
    }

    fn destroy_music(&mut self) {
        self.stop_music();
        /*         if (gMusic != NULL)
        {
            Mix_FreeMusic(gMusic);
        }

        gMusic = NULL;*/
    }

    fn load_sounds(&mut self) {
        if self.g_is_audio_initialized == false {
            return;
        }

        let effects_suffix = match self.snd_type {
            SoundType::Roland | SoundType::SoundBlaster => K_BLASTER_SOUND_FILENAME_SUFFIX,
            SoundType::Adlib => K_ADLIB_SOUND_FILENAME_SUFFIX,
            SoundType::InternalSamples => K_SAMPLES_SOUND_FILENAME_SUFFIX,
            SoundType::InternalStandard => K_STANDARD_SOUND_FILENAME_SUFFIX,
            SoundType::None => return,
        };

        for i in 0..SOUND_EFFECT_COUNT {
            let filename = format!(
                "{}/{}-{}.wav",
                K_BASE_AUDIO_FOLDER, G_SOUND_EFFECT_NAMES[i], effects_suffix
            );
            let chunk = sdl2::rwops::RWops::from_file(filename, "r").unwrap();
            let chunk = chunk.load_wav().unwrap();
            self.g_sound_effect_chunks[i] = Some(chunk);
        }
    }

    fn destroy_sounds(&mut self) {
        for i in 0..SOUND_EFFECT_COUNT {
            /*if (gSoundEffectChunks[i] != NULL)
            {
                Mix_FreeChunk(gSoundEffectChunks[i]);
            }
            gSoundEffectChunks[i] = NULL;*/
        }
    }
}
