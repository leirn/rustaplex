use crate::game::globals::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub struct Graphics {
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
}

impl Graphics {
    pub fn init() -> Graphics {
        let mut graphics = Graphics {
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
            g_palettes: Box::new([ColorPalette::default(); K_NUMBER_OF_PALETTES]),
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
        let path = format!("{}/{}T", RESSOURCES_PATH, G_CHARS6_DAT_FILENAME);
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

        let path = format!("{}/{}T", RESSOURCES_PATH, G_CHARS8_DAT_FILENAME);
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

    fn read_and_render_title_dat(&self) {}

    fn read_and_render_title1_dat(&self) {}

    /// Load TITLE2.DAT
    fn read_title2_dat(&mut self) {
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

        let mut file_data = [0_u8; K_SCREEN_WIDTH / 2];
        for y in 0..K_PANEL_BITMAP_HEIGHT {
            file.read(&mut file_data)
                .expect(format!("Error while reading {}", G_TITLE2_DAT_FILENAME).as_str());

            for x in 0..K_PANEL_BITMAP_WIDTH {
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

    fn convert_palette_data_to_palette(palette_data: ColorPaletteData) -> ColorPalette {
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

    fn draw_level_viewport(&self) {}
}

#[derive(Copy, Clone, Default)]
struct Color {
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
}

pub const K_SCREEN_WIDTH: usize = 320;
pub const K_SCREEN_HEIGHT: usize = 200;
const K_FULL_SCREEN_FRAMEBUFFER_LENGTH: usize = K_SCREEN_WIDTH * K_SCREEN_HEIGHT;
const K_FULL_SCREEN_BITMAP_LENGTH: usize = K_SCREEN_WIDTH * K_SCREEN_HEIGHT / 2; // They use 4 bits to encode pixels

const K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT: usize = 64;
const K_BITMAP_FONT_LENGTH: usize = K_NUMBER_OF_CHARACTERS_IN_BITMAP_FONT * 8;
const K_NUMBER_OF_COLORS: usize = 16;

const K_NUMBER_OF_PALETTES: usize = 4;
const K_PALETTE_DATA_SIZE: usize = 64;

const K_MOVING_BITMAP_WIDTH: usize = 320;
const K_MOVING_BITMAP_HEIGHT: usize = 462;
const K_FIXED_BITMAP_WIDTH: usize = 460;
const K_FIXED_BITMAP_HEIGHT: usize = 16;
const K_PANEL_BITMAP_WIDTH: usize = 320;
const K_PANEL_BITMAP_HEIGHT: usize = 24;

type ColorPalette = [Color; K_NUMBER_OF_COLORS];
type ColorPaletteData = [u8; K_PALETTE_DATA_SIZE];

const G_BLACK_PALETTE: ColorPalette = [Color::new(); K_NUMBER_OF_COLORS];

const G_TITLE_PALETTE_DATA: ColorPaletteData = [
    0x02, 0x03, 0x05, 0x00, 0x0D, 0x0A, 0x04, 0x0C, 0x02, 0x06, 0x06, 0x02, 0x03, 0x09, 0x09, 0x03,
    0x0B, 0x08, 0x03, 0x06, 0x02, 0x07, 0x07, 0x0A, 0x08, 0x06, 0x0D, 0x09, 0x06, 0x04, 0x0B, 0x01,
    0x09, 0x01, 0x00, 0x04, 0x0B, 0x01, 0x00, 0x04, 0x0D, 0x01, 0x00, 0x0C, 0x0F, 0x01, 0x00, 0x0C,
    0x0F, 0x06, 0x04, 0x0C, 0x02, 0x05, 0x06, 0x08, 0x0F, 0x0C, 0x06, 0x0E, 0x0C, 0x0C, 0x0D, 0x0E,
];

const G_TITLE1_PALETTE_DATA: ColorPaletteData = [
    0x00, 0x00, 0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F, 0x08, 0x08, 0x08, 0x08, 0x0A, 0x0A, 0x0A, 0x07,
    0x0A, 0x0A, 0x0A, 0x07, 0x0B, 0x0B, 0x0B, 0x07, 0x0E, 0x01, 0x01, 0x04, 0x09, 0x09, 0x09, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x09, 0x00, 0x00, 0x04, 0x0B, 0x00, 0x00, 0x0C,
    0x08, 0x08, 0x08, 0x08, 0x05, 0x05, 0x05, 0x08, 0x06, 0x06, 0x06, 0x08, 0x08, 0x08, 0x08, 0x08,
];

const G_TITLE2_PALETTE_DATA: ColorPaletteData = [
    0x00, 0x00, 0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F, 0x06, 0x06, 0x06, 0x08, 0x0A, 0x0A, 0x0A, 0x07,
    0x0A, 0x0A, 0x0A, 0x07, 0x0B, 0x0B, 0x0B, 0x07, 0x0E, 0x01, 0x01, 0x04, 0x09, 0x09, 0x09, 0x07,
    0x01, 0x03, 0x07, 0x00, 0x08, 0x08, 0x08, 0x08, 0x09, 0x00, 0x00, 0x04, 0x0B, 0x00, 0x00, 0x0C,
    0x00, 0x02, 0x0A, 0x01, 0x05, 0x05, 0x05, 0x08, 0x06, 0x06, 0x06, 0x08, 0x08, 0x08, 0x08, 0x07,
];
