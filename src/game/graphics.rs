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

use crate::game::button_borders::{ButtonBorderLineDescriptor, ButtonBorderLineType};
use crate::game::globals::*;
use crate::game::video::Video;
use sdl2::pixels::{Color, Palette};
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

pub struct Graphics<'a> {
    pub video: Rc<RefCell<Video<'a>>>,
    g_menu_bitmap_data: Box<[u8; K_FULL_SCREEN_BITMAP_LENGTH]>,
    g_back_bitmap_data: Box<[u8; K_FULL_SCREEN_BITMAP_LENGTH]>,
    g_controls_bitmap_data: Box<[u8; K_FULL_SCREEN_BITMAP_LENGTH]>,
    g_gfx_bitmap_data: Box<[u8; K_FULL_SCREEN_BITMAP_LENGTH]>,
    g_chars_6_bitmap_font: Box<[u8; K_BITMAP_FONT_LENGTH]>,
    g_chars_8_bitmap_font: Box<[u8; K_BITMAP_FONT_LENGTH]>,
    g_moving_decoded_bitmap_data: Box<[u8; K_MOVING_BITMAP_HEIGHT * K_MOVING_BITMAP_WIDTH]>,
    g_fixed_decoded_bitmap_data: Box<[u8; K_MOVING_BITMAP_HEIGHT * K_MOVING_BITMAP_WIDTH]>,
    g_panel_decoded_bitmap_data: Box<[u8; K_PANEL_BITMAP_HEIGHT * K_PANEL_BITMAP_WIDTH]>,
    g_panel_rendered_bitmap_data: Box<[u8; K_PANEL_BITMAP_HEIGHT * K_PANEL_BITMAP_WIDTH]>,
    g_title2_decoded_bitmap_data: Box<[u8; K_FULL_SCREEN_FRAMEBUFFER_LENGTH]>,
    g_palettes: Box<[ColorPalette; K_NUMBER_OF_PALETTES]>,
    g_current_palette: ColorPalette,
    g_should_show_fps: bool,
    g_should_limit_fps: bool,
    s_number_of_frames: u32,
    g_frame_rate_reference_time: u32,
    g_frame_rate: f32,
    s_last_frame_time: u32,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
    g_render_delta_time: u32,
}

impl Graphics<'_> {
    pub fn init(
        video: Rc<RefCell<Video<'static>>>,
        sdl_context: Rc<RefCell<sdl2::Sdl>>,
    ) -> Graphics<'static> {
        let mut graphics = Graphics {
            video: video,
            g_menu_bitmap_data: Box::new([0; K_FULL_SCREEN_BITMAP_LENGTH]),
            g_back_bitmap_data: Box::new([0; K_FULL_SCREEN_BITMAP_LENGTH]),
            g_controls_bitmap_data: Box::new([0; K_FULL_SCREEN_BITMAP_LENGTH]),
            g_gfx_bitmap_data: Box::new([0; K_FULL_SCREEN_BITMAP_LENGTH]),
            g_chars_6_bitmap_font: Box::new([0; K_BITMAP_FONT_LENGTH]),
            g_chars_8_bitmap_font: Box::new([0; K_BITMAP_FONT_LENGTH]),
            g_moving_decoded_bitmap_data: Box::new(
                [0; K_MOVING_BITMAP_HEIGHT * K_MOVING_BITMAP_WIDTH],
            ),
            g_fixed_decoded_bitmap_data: Box::new(
                [0; K_MOVING_BITMAP_HEIGHT * K_MOVING_BITMAP_WIDTH],
            ),
            g_panel_decoded_bitmap_data: Box::new(
                [0; K_PANEL_BITMAP_HEIGHT * K_PANEL_BITMAP_WIDTH],
            ),
            g_panel_rendered_bitmap_data: Box::new(
                [0; K_PANEL_BITMAP_HEIGHT * K_PANEL_BITMAP_WIDTH],
            ),
            g_title2_decoded_bitmap_data: Box::new([0; K_FULL_SCREEN_FRAMEBUFFER_LENGTH]),
            g_palettes: Box::new([G_BLACK_PALETTE; K_NUMBER_OF_PALETTES]),
            g_current_palette: G_BLACK_PALETTE,
            g_should_show_fps: false,
            g_should_limit_fps: true,
            s_number_of_frames: 0,
            g_frame_rate_reference_time: 0,
            g_frame_rate: 0.0,
            s_last_frame_time: 0,
            sdl_context: sdl_context,
            g_render_delta_time: 0,
        };
        graphics.load_murphy_sprites();
        graphics.read_palettes_dat();
        graphics.read_bitmap_fonts();
        graphics.read_panel_dat();
        graphics.read_menu_dat();
        graphics.read_controls_dat();
        graphics.read_back_dat();
        graphics.read_gfx_dat();
        graphics.read_title2_dat();
        graphics
    }

    /// Load MENU.DAT file
    fn read_menu_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_MENU_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_MENU_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_MENU_DAT_FILENAME).as_str());
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_FULL_SCREEN_BITMAP_LENGTH {
                    panic!("{} has not the right size", G_MENU_DAT_FILENAME);
                }
            }
            Err(err) => panic!("Error while opening {} : {}", G_MENU_DAT_FILENAME, err),
        }

        self.g_menu_bitmap_data =
            Box::new(data[0..K_FULL_SCREEN_BITMAP_LENGTH].try_into().unwrap());
    }

    /// Load MOVING.DAT and FIXED.DAT
    /// IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT
    /// MOVING.DAT bitmap size is 320x462
    /// FIXED.DAT bitmap size is 640x16
    fn load_murphy_sprites(&mut self) {
        {
            // File scope for MOVING.DAT
            let path = format!("{}/{}", RESSOURCES_PATH, G_MOVING_DAT_FILENAME);
            let menu_file_path = Path::new(&path);
            match menu_file_path
                .try_exists()
                .expect(format!("Can't check existence of file {}", G_MOVING_DAT_FILENAME).as_str())
            {
                true => (),
                false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
            }
            let mut file = File::open(menu_file_path)
                .expect(format!("Error while opening {}", G_MOVING_DAT_FILENAME).as_str());

            for y in 0..K_MOVING_BITMAP_HEIGHT {
                let mut file_data = [0_u8; K_MOVING_BITMAP_WIDTH / 2];
                file.read(&mut file_data)
                    .expect(format!("Error while reading {}", G_MOVING_DAT_FILENAME).as_str());

                for x in 0..K_MOVING_BITMAP_WIDTH {
                    let dest_pixels_address = y * K_MOVING_BITMAP_WIDTH + x;
                    let source_pixel_address = x / 8;
                    let source_pixel_bit_position = 7 - (x % 8);

                    let b: u8 =
                        (file_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                    let g: u8 =
                        (file_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                    let r: u8 =
                        (file_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                    let i: u8 =
                        (file_data[source_pixel_address + 120] >> source_pixel_bit_position) & 0x1;

                    let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                    self.g_moving_decoded_bitmap_data[dest_pixels_address] = final_color;
                }
            }
        }

        {
            // File scope for FIXED.DAT
            let path = format!("{}/FIXED.DAT", RESSOURCES_PATH);
            let menu_file_path = Path::new(&path);
            match menu_file_path
                .try_exists()
                .expect(format!("Can't check existence of file {}", G_FIXED_DAT_FILENAME).as_str())
            {
                true => (),
                false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
            }
            let mut file = File::open(menu_file_path)
                .expect(format!("Error while opening {}", G_FIXED_DAT_FILENAME).as_str());

            let mut bitmap_data = [0_u8; K_FIXED_BITMAP_WIDTH * K_FIXED_BITMAP_HEIGHT / 2];
            file.read(&mut bitmap_data)
                .expect(format!("Error while reading {}", G_FIXED_DAT_FILENAME).as_str());
            for y in 0..K_FIXED_BITMAP_HEIGHT {
                for x in 0..K_FIXED_BITMAP_WIDTH {
                    let dest_pixels_address = y * K_FIXED_BITMAP_WIDTH + x;
                    let source_pixel_address = y * K_FIXED_BITMAP_WIDTH / 2 + x / 8;
                    let source_pixel_bit_position = 7 - (x % 8);

                    let b: u8 =
                        (bitmap_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                    let g: u8 =
                        (bitmap_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                    let r: u8 =
                        (bitmap_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                    let i: u8 = (bitmap_data[source_pixel_address + 120]
                        >> source_pixel_bit_position)
                        & 0x1;

                    let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                    self.g_fixed_decoded_bitmap_data[dest_pixels_address] = final_color;
                }
            }
        }
    }

    fn read_panel_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_PANEL_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_PANEL_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_PANEL_DAT_FILENAME).as_str());

        let mut bitmap_data = [0_u8; K_PANEL_BITMAP_WIDTH * K_PANEL_BITMAP_HEIGHT / 2];
        file.read(&mut bitmap_data)
            .expect(format!("Error while reading {}", G_PANEL_DAT_FILENAME).as_str());
        for y in 0..K_PANEL_BITMAP_HEIGHT {
            for x in 0..K_PANEL_BITMAP_WIDTH {
                let dest_pixels_address = y * K_PANEL_BITMAP_WIDTH + x;
                let source_pixel_address = y * K_PANEL_BITMAP_WIDTH / 2 + x / 8;
                let source_pixel_bit_position = 7 - (x % 8);

                let b: u8 =
                    (bitmap_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                let g: u8 =
                    (bitmap_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                let r: u8 =
                    (bitmap_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                let i: u8 =
                    (bitmap_data[source_pixel_address + 120] >> source_pixel_bit_position) & 0x1;

                let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                self.g_panel_decoded_bitmap_data[dest_pixels_address] = final_color;
            }
        }
    }

    /// Load BACK.DAT file
    fn read_back_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_BACK_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_BACK_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_BACK_DAT_FILENAME).as_str());
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_FULL_SCREEN_BITMAP_LENGTH {
                    panic!("{} has not the right size", G_BACK_DAT_FILENAME);
                }
            }
            Err(err) => panic!("Error while opening {} : {}", G_BACK_DAT_FILENAME, err),
        }

        self.g_back_bitmap_data =
            Box::new(data[0..K_FULL_SCREEN_BITMAP_LENGTH].try_into().unwrap());
    }

    /// Load chars bitmap
    fn read_bitmap_fonts(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_CHARS6_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_CHARS6_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_CHARS6_DAT_FILENAME).as_str());
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_BITMAP_FONT_LENGTH {
                    panic!("{} has not the right size", G_CHARS6_DAT_FILENAME);
                }
            }
            Err(err) => panic!("Error while opening CHAR6.DAT : {}", err),
        }
        self.g_chars_6_bitmap_font = Box::new(data[0..K_BITMAP_FONT_LENGTH].try_into().unwrap());

        let path = format!("{}/{}", RESSOURCES_PATH, G_CHARS8_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_CHARS8_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_CHARS8_DAT_FILENAME).as_str());
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_BITMAP_FONT_LENGTH {
                    panic!("{} has not the right size", G_CHARS8_DAT_FILENAME);
                }
            }
            Err(err) => panic!("Error while opening CHARS8.DAT : {}", err),
        }
        self.g_chars_8_bitmap_font = Box::new(data[0..K_BITMAP_FONT_LENGTH].try_into().unwrap());
    }

    pub fn read_and_render_title_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_TITLE_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_TITLE_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_TITLE_DAT_FILENAME).as_str());

        const K_BYTES_PER_ROW: usize = K_SCREEN_WIDTH / 2;
        let mut file_data = [0_u8; K_BYTES_PER_ROW];

        for y in 0..K_SCREEN_HEIGHT {
            file.read(&mut file_data)
                .expect(format!("Error while reading {}", G_TITLE_DAT_FILENAME).as_str());

            for x in 0..K_SCREEN_WIDTH {
                let dest_pixels_address = y * K_SCREEN_WIDTH + x;
                let source_pixel_address = x / 8;
                let source_pixel_bit_position = 7 - (x % 8);

                let b: u8 =
                    (file_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                let g: u8 =
                    (file_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                let r: u8 =
                    (file_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                let i: u8 =
                    (file_data[source_pixel_address + 120] >> source_pixel_bit_position) & 0x1;

                let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                self.video
                    .borrow_mut()
                    .set_pixel(dest_pixels_address, final_color);
            }
        }
    }

    pub fn read_and_render_title1_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_TITLE1_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_TITLE1_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_TITLE1_DAT_FILENAME).as_str());

        const K_BYTES_PER_ROW: usize = K_SCREEN_WIDTH / 2;
        let mut file_data = [0_u8; K_BYTES_PER_ROW];

        for y in 0..K_SCREEN_HEIGHT {
            file.read(&mut file_data)
                .expect(format!("Error while reading {}", G_TITLE1_DAT_FILENAME).as_str());

            for x in 0..K_SCREEN_WIDTH {
                let dest_pixels_address = y * K_SCREEN_WIDTH + x;
                let source_pixel_address = x / 8;
                let source_pixel_bit_position = 7 - (x % 8);

                let b: u8 =
                    (file_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                let g: u8 =
                    (file_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                let r: u8 =
                    (file_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                let i: u8 =
                    (file_data[source_pixel_address + 120] >> source_pixel_bit_position) & 0x1;

                let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                self.video
                    .borrow_mut()
                    .set_pixel(dest_pixels_address, final_color);
            }
        }
    }

    /// Load TITLE2.DAT
    pub fn read_title2_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_TITLE2_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_TITLE2_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_TITLE2_DAT_FILENAME).as_str());

        const K_BYTES_PER_ROW: usize = K_SCREEN_WIDTH / 2;
        let mut file_data = [0_u8; K_BYTES_PER_ROW];

        for y in 0..K_SCREEN_HEIGHT {
            file.read(&mut file_data)
                .expect(format!("Error while reading {}", G_TITLE2_DAT_FILENAME).as_str());

            for x in 0..K_SCREEN_WIDTH {
                let dest_pixels_address = y * K_SCREEN_WIDTH + x;
                let source_pixel_address = x / 8;
                let source_pixel_bit_position = 7 - (x % 8);

                let b: u8 =
                    (file_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                let g: u8 =
                    (file_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                let r: u8 =
                    (file_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                let i: u8 =
                    (file_data[source_pixel_address + 120] >> source_pixel_bit_position) & 0x1;

                let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                self.g_title2_decoded_bitmap_data[dest_pixels_address] = final_color;
            }
        }
    }

    /// Load GFX.DAT
    fn read_gfx_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_GFX_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_GFX_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_GFX_DAT_FILENAME).as_str());
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_FULL_SCREEN_BITMAP_LENGTH {
                    panic!("{} has not the right size", G_GFX_DAT_FILENAME);
                }
            }
            Err(err) => panic!("Error while opening {} : {}", G_GFX_DAT_FILENAME, err),
        }

        self.g_gfx_bitmap_data = Box::new(data[0..K_FULL_SCREEN_BITMAP_LENGTH].try_into().unwrap());
    }

    pub fn convert_palette_data_to_palette(palette_data: ColorPaletteData) -> ColorPalette {
        let k_exponent = 4;

        let mut palette: ColorPalette = [Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }; K_NUMBER_OF_COLORS];
        for i in 0..K_NUMBER_OF_COLORS {
            palette[i].r = palette_data[i * 4 + 0] << k_exponent;
            palette[i].g = palette_data[i * 4 + 1] << k_exponent;
            palette[i].b = palette_data[i * 4 + 2] << k_exponent;
            palette[i].a = palette_data[i * 4 + 3] << k_exponent;
        }
        palette
    }

    /// Load PALETTES.DAT
    fn read_palettes_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_PALETTES_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_PALETTES_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }

        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_PALETTES_DAT_FILENAME).as_str());

        for i in 0..K_NUMBER_OF_PALETTES {
            let mut palette: ColorPaletteData = [0; K_PALETTE_DATA_SIZE];
            match file.read(&mut palette) {
                Ok(number_of_bytes_read) => {
                    if number_of_bytes_read < K_PALETTE_DATA_SIZE {
                        panic!("{} has not the right size", G_PALETTES_DAT_FILENAME);
                    }
                }
                Err(err) => panic!("Error while opening {} : {}", G_PALETTES_DAT_FILENAME, err),
            }

            self.g_palettes[i] = Graphics::convert_palette_data_to_palette(palette);
        }
    }

    pub fn fade_to_palette(&mut self, palette: ColorPalette) {
        // Parameters:
        // si -> points to the first color of the palette to fade to

        let mut intermediate_palette: ColorPalette = G_BLACK_PALETTE;

        // The original animation had 64 steps, and the game was written to run in 70Hz displays
        const K_FADE_DURATION: u32 = 64 * 1000 / 70; // ~914 ms
        let mut fade_time: u32 = 0;

        self.start_tracking_tender_delta_time();

        // for (uint8_t step = 0; step < totalSteps; ++step)
        while fade_time < K_FADE_DURATION {
            fade_time += self.update_render_delta_time();
            fade_time = std::cmp::min(fade_time, K_FADE_DURATION);

            let animation_factor = fade_time as f64 / K_FADE_DURATION as f64;
            let complementary_animation_factor = 1.0 - animation_factor;

            for i in 0..K_NUMBER_OF_COLORS {
                let r = (palette[i].r as f64 * animation_factor)
                    + (self.g_current_palette[i].r as f64 * complementary_animation_factor);
                let g = (palette[i].g as f64 * animation_factor)
                    + (self.g_current_palette[i].g as f64 * complementary_animation_factor);
                let b = (palette[i].b as f64 * animation_factor)
                    + (self.g_current_palette[i].b as f64 * complementary_animation_factor);

                intermediate_palette[i] = Color {
                    r: r as u8,
                    g: g as u8,
                    b: b as u8,
                    a: 255,
                };
            }

            self.video
                .borrow_mut()
                .set_color_palette(intermediate_palette);

            self.video_loop();
        }

        self.set_palette(palette);
    }

    /// Load CONTROLS.DAT
    fn read_controls_dat(&mut self) {
        let path = format!("{}/{}", RESSOURCES_PATH, G_CONTROLS_DAT_FILENAME);
        let menu_file_path = Path::new(&path);
        match menu_file_path
            .try_exists()
            .expect(format!("Can't check existence of file {}", G_CONTROLS_DAT_FILENAME).as_str())
        {
            true => (),
            false => panic!("{:?} doesn't exists", menu_file_path.canonicalize()),
        }
        let mut file = File::open(menu_file_path)
            .expect(format!("Error while opening {}", G_CONTROLS_DAT_FILENAME).as_str());
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(number_of_bytes_read) => {
                if number_of_bytes_read < K_FULL_SCREEN_BITMAP_LENGTH {
                    panic!("{} has not the right size", G_CONTROLS_DAT_FILENAME);
                }
            }
            Err(err) => panic!("Error while opening {} : {}", G_CONTROLS_DAT_FILENAME, err),
        }

        self.g_controls_bitmap_data =
            Box::new(data[0..K_FULL_SCREEN_BITMAP_LENGTH].try_into().unwrap());
    }

    pub fn set_palette(&mut self, palette: ColorPalette) {
        self.video.borrow_mut().set_color_palette(palette);
        self.g_current_palette = palette.clone();
    }

    pub fn video_loop(&mut self) {
        if self.g_should_show_fps {
            // TODO
            /*
            char frameRateString[5] = "";
            sprintf(frameRateString, "%4.1f", MIN(gFrameRate, 999.9)); // Don't show more than 999.9 FPS, not necessary

            drawTextWithChars6FontWithOpaqueBackground(0, 0, 6, frameRateString);*/
        }

        //handleSystemEvents(); // Make sure the app stays responsive

        self.video.borrow_mut().render();
        self.video.borrow_mut().present();

        if self.g_should_limit_fps {
            self.limit_fps();
        }

        self.s_number_of_frames += 1;

        if self.g_frame_rate_reference_time == 0 {
            self.g_frame_rate_reference_time = self.get_time();
        } else {
            let difference = (self.get_time() - self.g_frame_rate_reference_time) as f32;

            if difference > 1000.0 {
                self.g_frame_rate = self.s_number_of_frames as f32 * 1000_f32 / difference;
                self.s_number_of_frames = 0;
                self.g_frame_rate_reference_time = self.get_time();
            }
        }
    }

    fn limit_fps(&mut self) {
        const K_MAXIMUM_FPS: f32 = 70.0;
        const K_FRAME_DURATION: f32 = 1000.0 / K_MAXIMUM_FPS;
        self.s_last_frame_time = 0;

        if self.s_last_frame_time != 0 {
            let duration = (self.get_time() - self.s_last_frame_time) as f32;
            if duration < K_FRAME_DURATION {
                let sleep_duration =
                    std::time::Duration::from_millis((K_FRAME_DURATION - duration) as u64);
                std::thread::sleep(sleep_duration);
            }
        }

        self.s_last_frame_time = self.get_time();
    }

    fn get_time(&self) -> u32 {
        self.sdl_context.borrow().timer().unwrap().ticks()
    }

    pub fn start_tracking_tender_delta_time(&mut self) {
        self.g_render_delta_time = self.get_time();
    }

    pub fn update_render_delta_time(&mut self) -> u32 {
        let duration = self.get_time() - self.g_render_delta_time;
        self.g_render_delta_time = self.get_time();
        duration
    }

    pub fn draw_text_with_chars6_font_with_opaque_background(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        if text.len() == 0 {
            return;
        }
        for idx in 0..text.len() {
            //for character in text.bytes().into_iter() {
            let character = text.as_bytes()[idx];
            if character == 0x0a {
                // equivalent to '\n'
                return;
            }
            // ' ' = 0x20 = 32, and is first ascii that can be represented.
            // This line converts the ascii from the string to the index in the font
            let bitmap_character_index = character - 0x20;

            for y in 0..K_BITMAP_FONT_CHARACTER_HEIGHT {
                for x in 0..K_BITMAP_FONT_CHARACTER_6_WIDTH {
                    let bitmap_character_row = self.g_chars_6_bitmap_font[bitmap_character_index
                        as usize
                        + y * K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT];
                    let pixel_value = (bitmap_character_row >> (7 - x)) & 0x1;

                    // 6 is the wide (in pixels) of this font
                    let dest_address = (dest_y + y) * K_SCREEN_WIDTH
                        + (idx * K_BITMAP_FONT_CHARACTER_6_WIDTH + dest_x + x);
                    self.video
                        .borrow_mut()
                        .set_pixel(dest_address, color * pixel_value);
                }
            }
        }
    }

    pub fn draw_text_with_chars6_font_with_transparent_background(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        if text.len() == 0 {
            return;
        }
        for idx in 0..text.len() {
            //for character in text.bytes().into_iter() {
            let character = text.as_bytes()[idx];
            if character == 0x0a {
                // equivalent to '\n'
                return;
            }
            // ' ' = 0x20 = 32, and is first ascii that can be represented.
            // This line converts the ascii from the string to the index in the font
            let bitmap_character_index = character - 0x20;

            for y in 0..K_BITMAP_FONT_CHARACTER_HEIGHT {
                for x in 0..K_BITMAP_FONT_CHARACTER_6_WIDTH {
                    let bitmap_character_row = self.g_chars_6_bitmap_font[bitmap_character_index
                        as usize
                        + y * K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT];
                    let pixel_value = (bitmap_character_row >> (7 - x)) & 0x1;

                    if pixel_value == 1 {
                        // 6 is the wide (in pixels) of this font
                        let dest_address = (dest_y + y) * K_SCREEN_WIDTH
                            + (idx * K_BITMAP_FONT_CHARACTER_6_WIDTH + dest_x + x);
                        self.video
                            .borrow_mut()
                            .set_pixel(dest_address, color * pixel_value);
                    }
                }
            }
        }
    }

    pub fn draw_text_with_chars8_font_to_buffer(
        &mut self,
        buffer: DrawTextBuffer,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        if text.len() == 0 {
            return;
        }

        for idx in 0..text.len() {
            let character = text.as_bytes()[idx];

            if character == 0x0a {
                // equivalent to '\n'
                return;
            }

            // ' ' = 0x20 = 32, and is first ascii that can be represented.
            // This line converts the ascii from the string to the index in the font
            //
            let bitmap_character_index = character - 0x20;

            for y in 0..K_BITMAP_FONT_CHARACTER_HEIGHT {
                for x in 0..K_BITMAP_FONT_CHARACTER_8_WIDTH {
                    let bitmap_character_row = self.g_chars_8_bitmap_font[bitmap_character_index
                        as usize
                        + y * K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT];
                    let pixel_value = (bitmap_character_row >> (7 - x)) & 0x1;

                    // 6 is the wide (in pixels) of this font
                    let dest_address = (dest_y + y) * K_SCREEN_WIDTH
                        + (idx * K_BITMAP_FONT_CHARACTER_8_WIDTH + dest_x + x);
                    match buffer {
                        // TODO : Write to right buffer
                        DrawTextBuffer::G_SCREEN_PIXEL => (),
                        DrawTextBuffer::G_PANEL_RENDERED_BITMAP_DATA => (), //buffer[dest_address] = color * pixel_value;
                    }
                }
            }
        }
    }

    pub fn draw_text_with_chars8_font(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        self.draw_text_with_chars8_font_to_buffer(
            DrawTextBuffer::G_SCREEN_PIXEL,
            dest_x,
            dest_y,
            color,
            text,
        );
    }

    pub fn draw_text_with_chars8_font_to_game_panel(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        color: u8,
        text: String,
    ) {
        self.draw_text_with_chars8_font_to_buffer(
            DrawTextBuffer::G_PANEL_RENDERED_BITMAP_DATA,
            dest_x,
            dest_y,
            color,
            text,
        );
    }

    pub fn open_credits_block(&mut self) {
        const K_EDGE_WIDTH: u32 = 13;
        const K_EDGE_HEIGHT: u32 = 148;
        const K_EDGE_STEP: u32 = 4;
        const K_EDGE_TOP_Y: u32 = 26;
        const K_NUMBER_OF_FRAMES: u32 = 60;

        const K_ANIMATION_DURATION: u32 = K_NUMBER_OF_FRAMES * 1000 / 70; // ~429 ms

        let mut animation_time: u32 = 0;

        const K_INITIAL_LEFT_EDGE_X: u32 = 147;
        const K_INITIAL_RIGHT_EDGE_X: u32 = K_INITIAL_LEFT_EDGE_X + K_EDGE_WIDTH + 1;

        const K_EDGE_ANIMATION_DISTANCE: u32 = (K_EDGE_STEP * K_NUMBER_OF_FRAMES) / 2 + 1;

        let mut left_edge_x = K_INITIAL_LEFT_EDGE_X;
        let mut right_edge_x = K_INITIAL_RIGHT_EDGE_X;

        self.start_tracking_tender_delta_time();

        while animation_time < K_ANIMATION_DURATION {
            animation_time += self.update_render_delta_time();
            animation_time = std::cmp::min(animation_time, K_ANIMATION_DURATION);

            let animation_factor = animation_time as f64 / K_ANIMATION_DURATION as f64;

            let previous_left_edge_x = left_edge_x;
            let previous_right_edge_x = right_edge_x;

            let distance = (K_EDGE_ANIMATION_DISTANCE as f64 * animation_factor) as u32;

            left_edge_x = K_INITIAL_LEFT_EDGE_X - distance;
            right_edge_x = K_INITIAL_RIGHT_EDGE_X + distance;

            let left_edge_step = previous_left_edge_x - left_edge_x;
            let right_edge_step = right_edge_x - previous_right_edge_x;

            // This loop moves both edges of the panel, and fills the inside of the panel with the contents of TITLE2.DAT
            for y in K_EDGE_TOP_Y..(K_EDGE_TOP_Y + K_EDGE_HEIGHT) {
                // Left edge
                for x in left_edge_x..(previous_left_edge_x + K_EDGE_WIDTH - left_edge_step) {
                    let addr = y as usize * K_SCREEN_WIDTH + x as usize;
                    let color = self
                        .video
                        .borrow_mut()
                        .get_pixel(addr + left_edge_step as usize);
                    self.video.borrow_mut().set_pixel(addr, color); // Move panel edge
                }

                // Content of visible panel unveiled by left edge
                for x in (left_edge_x + K_EDGE_WIDTH)..(previous_left_edge_x + K_EDGE_WIDTH + 1) {
                    let addr = y as usize * K_SCREEN_WIDTH + x as usize;
                    self.video
                        .borrow_mut()
                        .set_pixel(addr, self.g_title2_decoded_bitmap_data[addr]);
                }

                // Right edge
                let mut x = previous_right_edge_x;
                while x > right_edge_x + K_EDGE_WIDTH
                //for x in (rightEdgeX + kEdgeWidth)..previousRightEdgeX // --
                {
                    x -= 1;
                    let addr = y as usize * K_SCREEN_WIDTH + x as usize;
                    let color = self
                        .video
                        .borrow_mut()
                        .get_pixel(addr - right_edge_step as usize);
                    self.video.borrow_mut().set_pixel(addr, color); // Move panel edge
                }

                // Content of visible panel unveiled by right edge
                for x in previous_right_edge_x..right_edge_x {
                    let addr = y as usize * K_SCREEN_WIDTH + x as usize;
                    self.video
                        .borrow_mut()
                        .set_pixel(addr, self.g_title2_decoded_bitmap_data[addr]);
                }
            }

            self.video_loop();
        }

        let copy_offset = K_EDGE_TOP_Y as usize * K_SCREEN_WIDTH;
        //memcpy(gScreenPixels + copyOffset, gTitle2DecodedBitmapData + copyOffset, sizeof(gTitle2DecodedBitmapData) - copyOffset);
        for i in 0..(K_FULL_SCREEN_FRAMEBUFFER_LENGTH - copy_offset) {
            let color = self.video.borrow_mut().get_pixel(copy_offset);
            self.video.borrow_mut().set_pixel(copy_offset, color); // Move panel edge
        }

        let title2_palette = Graphics::convert_palette_data_to_palette(G_TITLE2_PALETTE_DATA);
        self.fade_to_palette(title2_palette);
    }

    pub fn draw_menu_background(&mut self) {
        for y in 0..K_SCREEN_HEIGHT {
            for x in 0..K_SCREEN_WIDTH {
                let dest_pixel_address = y * K_SCREEN_WIDTH + x;

                let source_pixel_address = y * K_SCREEN_WIDTH / 2 + x / 8;
                let source_pixel_bit_position = 7 - (x % 8);

                let b = (self.g_menu_bitmap_data[source_pixel_address + 0]
                    >> source_pixel_bit_position)
                    & 0x1;
                let g = (self.g_menu_bitmap_data[source_pixel_address + 40]
                    >> source_pixel_bit_position)
                    & 0x1;
                let r = (self.g_menu_bitmap_data[source_pixel_address + 80]
                    >> source_pixel_bit_position)
                    & 0x1;
                let i = (self.g_menu_bitmap_data[source_pixel_address + 120]
                    >> source_pixel_bit_position)
                    & 0x1;

                let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                self.video
                    .borrow_mut()
                    .set_pixel(dest_pixel_address, final_color);
            }
        }
    }

    pub fn get_palette(&mut self, palette: PaletteType) -> ColorPalette {
        match palette {
            PaletteType::InformationScreenPalette => self.g_palettes[0],
            PaletteType::GamePalette => self.g_palettes[1],
            PaletteType::ControlsPalette => self.g_palettes[2],
            PaletteType::GameDimmedPalette => self.g_palettes[3],
        }
    }

    pub fn draw_back_background(&mut self) {
        self.draw_full_screen_bitmap(BitmapType::Background, DestinationSurface::Screen);
    }

    fn draw_full_screen_bitmap(&mut self, bitmap: BitmapType, dest: DestinationSurface) {
        let bitmap_data = match bitmap {
            BitmapType::Background => &self.g_back_bitmap_data,
            BitmapType::Control => &self.g_controls_bitmap_data,
            BitmapType::Gfx => &self.g_gfx_bitmap_data,
            BitmapType::Menu => &self.g_menu_bitmap_data,
        };

        for y in 0..K_SCREEN_HEIGHT {
            for x in 0..K_SCREEN_WIDTH {
                let dest_pixel_address = y * K_SCREEN_WIDTH + x;

                let source_pixel_address = y * K_SCREEN_WIDTH / 2 + x / 8;
                let source_pixel_bit_position = 7 - (x % 8);

                let b = (bitmap_data[source_pixel_address + 0] >> source_pixel_bit_position) & 0x1;
                let g = (bitmap_data[source_pixel_address + 40] >> source_pixel_bit_position) & 0x1;
                let r = (bitmap_data[source_pixel_address + 80] >> source_pixel_bit_position) & 0x1;
                let i =
                    (bitmap_data[source_pixel_address + 120] >> source_pixel_bit_position) & 0x1;

                let final_color = (b << 0) | (g << 1) | (r << 2) | (i << 3);

                match dest {
                    DestinationSurface::Screen => self
                        .video
                        .borrow_mut()
                        .set_pixel(dest_pixel_address, final_color),
                }
            }
        }
    }

    pub fn draw_main_menu_button_border(
        &mut self,
        border: &[ButtonBorderLineDescriptor],
        color: u8,
    ) {
        for i in 0..border.len() {
            let line = &border[i];

            for j in 0..line.length {
                let mut dest_address = 0;
                if line.button_type == ButtonBorderLineType::ButtonBorderLineTypeHorizontal {
                    dest_address = line.y * K_SCREEN_WIDTH as u16 + line.x + j;
                } else if line.button_type == ButtonBorderLineType::ButtonBorderLineTypeVertical {
                    dest_address = (line.y - j) * K_SCREEN_WIDTH as u16 + line.x;
                } else if line.button_type
                    == ButtonBorderLineType::ButtonBorderLineTypeBottomLeftToTopRightDiagonal
                {
                    dest_address = (line.y - j) * K_SCREEN_WIDTH as u16 + line.x + j;
                } else if line.button_type
                    == ButtonBorderLineType::ButtonBorderLineTypeTopLeftToBottomRightDiagonal
                {
                    dest_address = (line.y + j) * K_SCREEN_WIDTH as u16 + line.x + j;
                }
                self.video
                    .borrow_mut()
                    .set_pixel(dest_address as usize, color);
            }
        }

        //saveLastMouseAreaBitmap();
        //drawMouseCursor();
    }
}

enum DestinationSurface {
    Screen,
}

enum BitmapType {
    Background,
    Menu,
    Control,
    Gfx,
}

/*
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    pub const fn new() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}*/

pub const K_SCREEN_WIDTH: usize = 320;
pub const K_SCREEN_HEIGHT: usize = 200;
pub const K_FULL_SCREEN_FRAMEBUFFER_LENGTH: usize = K_SCREEN_WIDTH * K_SCREEN_HEIGHT;
const K_FULL_SCREEN_BITMAP_LENGTH: usize = K_SCREEN_WIDTH * K_SCREEN_HEIGHT / 2; // They use 4 bits to encode pixels

const K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT: usize = 64;
const K_BITMAP_FONT_LENGTH: usize = K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT * 8;
pub const K_NUMBER_OF_COLORS: usize = 16;

const K_NUMBER_OF_PALETTES: usize = 4;
const K_PALETTE_DATA_SIZE: usize = 64;

const K_MOVING_BITMAP_WIDTH: usize = 320;
const K_MOVING_BITMAP_HEIGHT: usize = 462;
const K_FIXED_BITMAP_WIDTH: usize = 460;
const K_FIXED_BITMAP_HEIGHT: usize = 16;
const K_PANEL_BITMAP_WIDTH: usize = 320;
const K_PANEL_BITMAP_HEIGHT: usize = 24;

const K_BITMAP_FONT_CHARACTER_HEIGHT: usize = 7;
const K_BITMAP_FONT_CHARACTER_6_WIDTH: usize = 6;
const K_BITMAP_FONT_CHARACTER_8_WIDTH: usize = 8;

pub type ColorPalette = [Color; K_NUMBER_OF_COLORS];
type ColorPaletteData = [u8; K_PALETTE_DATA_SIZE];

pub const G_BLACK_PALETTE: ColorPalette = [Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
}; K_NUMBER_OF_COLORS];

pub const G_TITLE_PALETTE_DATA: ColorPaletteData = [
    0x02, 0x03, 0x05, 0x00, 0x0D, 0x0A, 0x04, 0x0C, 0x02, 0x06, 0x06, 0x02, 0x03, 0x09, 0x09, 0x03,
    0x0B, 0x08, 0x03, 0x06, 0x02, 0x07, 0x07, 0x0A, 0x08, 0x06, 0x0D, 0x09, 0x06, 0x04, 0x0B, 0x01,
    0x09, 0x01, 0x00, 0x04, 0x0B, 0x01, 0x00, 0x04, 0x0D, 0x01, 0x00, 0x0C, 0x0F, 0x01, 0x00, 0x0C,
    0x0F, 0x06, 0x04, 0x0C, 0x02, 0x05, 0x06, 0x08, 0x0F, 0x0C, 0x06, 0x0E, 0x0C, 0x0C, 0x0D, 0x0E,
];

pub const G_TITLE1_PALETTE_DATA: ColorPaletteData = [
    0x00, 0x00, 0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F, 0x08, 0x08, 0x08, 0x08, 0x0A, 0x0A, 0x0A, 0x07,
    0x0A, 0x0A, 0x0A, 0x07, 0x0B, 0x0B, 0x0B, 0x07, 0x0E, 0x01, 0x01, 0x04, 0x09, 0x09, 0x09, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x09, 0x00, 0x00, 0x04, 0x0B, 0x00, 0x00, 0x0C,
    0x08, 0x08, 0x08, 0x08, 0x05, 0x05, 0x05, 0x08, 0x06, 0x06, 0x06, 0x08, 0x08, 0x08, 0x08, 0x08,
];

pub const G_TITLE2_PALETTE_DATA: ColorPaletteData = [
    0x00, 0x00, 0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F, 0x06, 0x06, 0x06, 0x08, 0x0A, 0x0A, 0x0A, 0x07,
    0x0A, 0x0A, 0x0A, 0x07, 0x0B, 0x0B, 0x0B, 0x07, 0x0E, 0x01, 0x01, 0x04, 0x09, 0x09, 0x09, 0x07,
    0x01, 0x03, 0x07, 0x00, 0x08, 0x08, 0x08, 0x08, 0x09, 0x00, 0x00, 0x04, 0x0B, 0x00, 0x00, 0x0C,
    0x00, 0x02, 0x0A, 0x01, 0x05, 0x05, 0x05, 0x08, 0x06, 0x06, 0x06, 0x08, 0x08, 0x08, 0x08, 0x07,
];

pub enum DrawTextBuffer {
    G_SCREEN_PIXEL,
    G_PANEL_RENDERED_BITMAP_DATA,
}

pub enum PaletteType {
    GamePalette,
    ControlsPalette,
    GameDimmedPalette,
    InformationScreenPalette,
}
