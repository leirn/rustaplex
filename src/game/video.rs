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

use super::graphics::G_BLACK_PALETTE;
use super::graphics::{self, K_SCREEN_HEIGHT, K_SCREEN_WIDTH};
use crate::game::globals;
use crate::game::graphics::ColorPalette;
use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{DisplayMode, FullscreenType};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
enum ScalingMode {
    ScalingModeAspectFit,
    ScalingModeAspectFill,
    ScalingModeIntegerFactor,
    ScalingModeFullscreen,
    ScalingModeAspectCorrect,
    ScalingModeCount,
}

pub const TEXTURE_ASPECT_RATIO: f64 = K_SCREEN_WIDTH as f64 / K_SCREEN_HEIGHT as f64;

pub struct Video<'a> {
    g_renderer: sdl2::render::Canvas<sdl2::video::Window>,
    g_scaling_mode: ScalingMode,
    g_color_palette: ColorPalette,
    g_screen_surface: Surface<'a>,
}

impl Video<'_> {
    pub fn init(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Video<'static> {
        let _video_subsystem = sdl_context.borrow_mut().video().unwrap();
        let display_mode = DisplayMode::new(
            PixelFormatEnum::Index8,
            K_SCREEN_WIDTH as i32,
            K_SCREEN_HEIGHT as i32,
            60,
        );
        let mut _window = _video_subsystem
            .window(
                globals::WINDOW_TITLE,
                graphics::K_SCREEN_WIDTH as u32,
                graphics::K_SCREEN_HEIGHT as u32,
            )
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .resizable()
            .build()
            .unwrap();

        _window.set_display_mode(display_mode).unwrap(); // Tries to align palette format with everything

        let mut _canvas = _window
            .into_canvas()
            .present_vsync()
            .index(Video::find_sdl_gl_driver().unwrap())
            .present_vsync()
            .build()
            .unwrap();

        let surface = Surface::new(
            K_SCREEN_WIDTH as u32,
            K_SCREEN_HEIGHT as u32,
            PixelFormatEnum::Index8,
        )
        .unwrap();

        _canvas
            .window_mut()
            .set_size(
                (3 * K_SCREEN_WIDTH).try_into().unwrap(),
                (3 * K_SCREEN_HEIGHT).try_into().unwrap(),
            )
            .unwrap();
        Video {
            //video_subsystem: _video_subsystem,
            g_renderer: _canvas,
            g_scaling_mode: ScalingMode::ScalingModeAspectFit,
            g_color_palette: G_BLACK_PALETTE,
            g_screen_surface: surface,
        }
    }

    pub fn update_window_viewport(&mut self) {
        let (window_height, window_width) = self.g_renderer.window().size();

        // If the scaling mode is fullscreen, use the window size
        if self.g_scaling_mode == ScalingMode::ScalingModeFullscreen {
            self.g_renderer.viewport().set_x(0);
            self.g_renderer.viewport().set_y(0);
            self.g_renderer.viewport().set_width(window_width);
            self.g_renderer.viewport().set_height(window_height);
            return;
        }

        let texture_aspect_ratio;
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectCorrect {
            texture_aspect_ratio = 4 as f64 / 3 as f64;
        } else {
            texture_aspect_ratio = TEXTURE_ASPECT_RATIO;
        }

        let mut max_viewport_width = window_width;
        let mut max_viewport_height = window_height;

        // For "integer factor" scaling, pick the highest integer factor that fits into the window
        if self.g_scaling_mode == ScalingMode::ScalingModeIntegerFactor {
            max_viewport_width =
                ((window_width as f64 / K_SCREEN_WIDTH as f64) * K_SCREEN_WIDTH as f64) as u32;
            max_viewport_height =
                ((window_height as f64 / K_SCREEN_HEIGHT as f64) * K_SCREEN_HEIGHT as f64) as u32;
        }

        // If the resulting viewport is too small, do proportional scaling according to the window size
        if max_viewport_width == 0 {
            max_viewport_width = window_width;
        }
        if max_viewport_height == 0 {
            max_viewport_height = window_height;
        }

        let screen_aspect_ratio = window_width as f64 / window_height as f64;
        let mut should_preserve_width = texture_aspect_ratio > screen_aspect_ratio;

        // The only difference between aspect fill and fit is that fit will leave black bars
        // and fill will crop the image.
        // TODO : does not seem to work, always in fit mode without preserving ratio
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectFill {
            should_preserve_width = !should_preserve_width;
        }

        if should_preserve_width {
            self.g_renderer
                .viewport()
                .set_x((window_width as i32 - max_viewport_width as i32) >> 1);
            self.g_renderer.viewport().set_width(max_viewport_width);
            let viewport_width = self.g_renderer.viewport().width();
            self.g_renderer
                .viewport()
                .set_height(viewport_width / texture_aspect_ratio as u32);
            let viewport_height = self.g_renderer.viewport().height();
            self.g_renderer
                .viewport()
                .set_y((window_height as i32 - viewport_height as i32) >> 1);
        } else {
            self.g_renderer
                .viewport()
                .set_y((window_height as i32 - max_viewport_height as i32) >> 1);
            self.g_renderer.viewport().set_height(max_viewport_height);
            let viewport_height = self.g_renderer.viewport().height();
            self.g_renderer
                .viewport()
                .set_width(viewport_height * texture_aspect_ratio as u32);
            let viewport_width = self.g_renderer.viewport().width();
            self.g_renderer
                .viewport()
                .set_x((window_width as i32 - viewport_width as i32) >> 1);
        }

        self.g_renderer.present();
    }

    pub fn get_scaling_mode(&self) -> ScalingMode {
        self.g_scaling_mode
    }

    pub fn set_scaling_mode(&mut self, mode: ScalingMode) {
        self.g_scaling_mode = mode;
        self.update_window_viewport();
    }

    pub fn toggle_fullscreen(&mut self) {
        self.set_fullscreen_mode(!self.get_fullscreen_mode());
    }

    fn set_fullscreen_mode(&mut self, fullscreen: bool) {
        let full_screen_type = match fullscreen {
            true => FullscreenType::True,
            false => FullscreenType::Desktop,
        };
        self.g_renderer
            .window_mut()
            .set_fullscreen(full_screen_type)
            .unwrap();
    }

    fn get_fullscreen_mode(&self) -> bool {
        let is_fullscreen = self.g_renderer.window().fullscreen_state();
        is_fullscreen == FullscreenType::True
    }

    /// Find SDL GL Driver to initiate SDL window
    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }

    pub fn set_pixel(&mut self, address: usize, color: u8) {
        self.g_screen_surface.without_lock_mut().unwrap()[address] = color;
    }

    pub fn get_pixel(&mut self, address: usize) -> u8 {
        self.g_screen_surface.without_lock_mut().unwrap()[address]
    }

    pub fn set_color_palette(&mut self, palette: ColorPalette) {
        self.g_color_palette = palette.clone();
        let palette = Palette::with_colors(&palette).unwrap();
        self.g_screen_surface.set_palette(&palette).unwrap();
    }

    pub fn render(&mut self) {
        //self.g_screen_surface
        //    .blit(None, &mut self.g_texture_surface.borrow_mut(), None);
        //SDL_BlitSurface(gScreenSurface, NULL, gTextureSurface, NULL);

        /*self.g_texture.update(
            None,
            self.g_texture_surface.without_lock().unwrap(),
            self.g_texture_surface.pitch() as usize,
        );*/
        //SDL_UpdateTexture(gTexture, NULL, gTextureSurface->pixels, gTextureSurface->pitch);

        self.g_renderer.clear();
        //SDL_RenderClear(gRenderer);
        let creator = self.g_renderer.texture_creator();
        let texture = self.g_screen_surface.as_texture(&creator).unwrap();

        self.g_renderer
            .copy(&texture, None, self.g_renderer.viewport());
        //SDL_RenderCopy(gRenderer, gTexture, NULL, &gWindowViewport);
    }

    pub fn present(&mut self) {
        self.g_renderer.present();
    }
}
