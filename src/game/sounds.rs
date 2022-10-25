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

enum SoundType {
    SoundTypeNone = 0,
    SoundTypeInternalStandard = 1,
    SoundTypeInternalSamples = 2,
    SoundTypeAdlib = 3,
    SoundTypeSoundBlaster = 4,
    SoundTypeRoland = 5,
}

enum SoundEffect {
    SoundEffectExplosion,
    SoundEffectInfotron,
    SoundEffectPush,
    SoundEffectFall,
    SoundEffectBug,
    SoundEffectBase,
    SoundEffectExit,
    SoundEffectCount,
}

pub struct Sounds {
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    pub is_music_enabled: bool,
    pub is_fx_enabled: bool,
    g_current_sound_priority: u8,
    g_current_sound_duration: u8,
}

impl Sounds {
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Sounds {
        Sounds {
            sdl_context: sdl_context,
            is_music_enabled: false,
            is_fx_enabled: false,
            g_current_sound_priority: 0,
            g_current_sound_duration: 0,
        }
    }
}

impl Sounds {
    pub fn sound_shutdown(&mut self) {
        self.stop_music_and_sounds();
    }

    pub fn activate_internal_standard_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(
            SoundType::SoundTypeInternalStandard,
            SoundType::SoundTypeInternalStandard,
        );
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_internal_samples_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(
            SoundType::SoundTypeInternalStandard,
            SoundType::SoundTypeInternalSamples,
        );
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_adlib_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::SoundTypeAdlib, SoundType::SoundTypeAdlib);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_sound_blaster_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::SoundTypeAdlib, SoundType::SoundTypeSoundBlaster);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_roland_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::SoundTypeRoland, SoundType::SoundTypeRoland);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn activate_combined_sound(&mut self) {
        self.stop_music_and_sounds();
        self.set_sound_type(SoundType::SoundTypeRoland, SoundType::SoundTypeSoundBlaster);
        self.play_music_if_needed();
        self.g_current_sound_priority = 0;
        self.g_current_sound_duration = 0;
    }

    pub fn stop_music_and_sounds(&mut self) {
        self.set_sound_type(SoundType::SoundTypeNone, SoundType::SoundTypeNone);
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

        self.play_sound_effect(SoundEffect::SoundEffectExplosion);
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

        self.play_sound_effect(SoundEffect::SoundEffectInfotron);
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

        self.play_sound_effect(SoundEffect::SoundEffectPush);
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

        self.play_sound_effect(SoundEffect::SoundEffectFall);
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

        self.play_sound_effect(SoundEffect::SoundEffectBug);
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

        self.play_sound_effect(SoundEffect::SoundEffectBase);
    }

    pub fn play_exit_sound(&mut self) {
        if self.is_fx_enabled == false {
            return;
        }

        self.g_current_sound_priority = 0xa;
        self.g_current_sound_duration = 0xfa;
        self.stop_music();

        self.play_sound_effect(SoundEffect::SoundEffectExit);
    }

    fn set_sound_type(&mut self, music_type: SoundType, effects_type: SoundType) {}

    fn play_sound_effect(&mut self, effects_type: SoundEffect) {}

    fn play_music(&mut self) {}

    fn stop_music(&mut self) {}
}
