use crate::grid;
use crate::ui::drawing::RaylibDrawHandle;
use raylib::prelude::RaylibDraw;
use raylib::*;

const PADDING: u16 = 3;
pub const WINDOW_SIZE: u16 = 9 * 90 + PADDING;
pub const TILE_SIZE: u16 = WINDOW_SIZE / 9;
pub fn draw_tiles(rldh: &mut RaylibDrawHandle, grid: &grid::Grid) {
    for row in 0..9 {
        for col in 0..9 {
            let x: u16 = col * TILE_SIZE;
            let y: u16 = row * TILE_SIZE;

            rldh.draw_rectangle(
                (x + PADDING) as i32,
                (y + PADDING) as i32,
                (TILE_SIZE - PADDING) as i32,
                (TILE_SIZE - PADDING) as i32,
                raylib::color::rcolor(0xFF, 0xFF, 0xFF, 0xDD),
            );
            rldh.draw_text(
                grid.tiles[(row * 9 + col) as usize]
                    .val
                    .to_ascii_lowercase()
                    .to_string()
                    .as_str(),
                (x + TILE_SIZE / 2 - PADDING * 3) as i32,
                (y + TILE_SIZE / 2 - PADDING * 8) as i32,
                60,
                raylib::color::rcolor(0, 0x77, 0xAA, 0xFF),
            );
        }
    }
}
