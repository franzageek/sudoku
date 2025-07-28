use crate::ui::drawing::RaylibDrawHandle;
use crate::{grid, tile};
use raylib::prelude::RaylibDraw;
use raylib::*;

const PADDING: u16 = 3;
pub const WINDOW_SIZE: u16 = 9 * 60 + PADDING;
pub const TILE_SIZE: u16 = WINDOW_SIZE / 9;
pub fn draw_tiles(rldh: &mut RaylibDrawHandle, grid: &grid::Grid) {
    rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
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
            if grid.tiles[(row * 9 + col) as usize].val > 0 {
                rldh.draw_text(
                    grid.tiles[(row * 9 + col) as usize]
                        .val
                        .to_ascii_lowercase()
                        .to_string()
                        .as_str(),
                    (x + TILE_SIZE / 2 - PADDING * 3) as i32,
                    (y + TILE_SIZE / 2 - PADDING * 8) as i32,
                    60,
                    if grid.tiles[(row * 9 + col) as usize].access == tile::Access::Default {
                        raylib::color::rcolor(0, 0x77, 0xAA, 0xFF)
                    } else if grid.tiles[(row * 9 + col) as usize].access == tile::Access::LPNPass {
                        raylib::color::rcolor(0xBB, 0x00, 0xBB, 0xFF)
                    } else if grid.tiles[(row * 9 + col) as usize].access == tile::Access::LRCPass {
                        raylib::color::rcolor(00, 0xDD, 0x55, 0xFF)
                    } else if grid.tiles[(row * 9 + col) as usize].access
                        == tile::Access::CouplePass
                    {
                        raylib::color::rcolor(0xFF, 0x55, 0x55, 0xFF)
                    } else {
                        raylib::color::rcolor(0xFF, 0xAA, 0x77, 0xFF)
                    },
                );
            } /*else {
            rldh.draw_text(
            "0",
            (x + TILE_SIZE / 2 - PADDING * 3) as i32,
            (y + TILE_SIZE / 2 - PADDING * 8) as i32,
            60,
            raylib::color::rcolor(0x8F, 0x8F, 0x8F, 0xFF)
            );
            }*/
        }
    }
}
