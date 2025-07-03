use crate::grid;
use crate::ui;
use raylib::prelude::RaylibDraw;
use raylib::*;

pub fn main_loop((mut handle, thread): (RaylibHandle, RaylibThread), grid: &mut grid::Grid) {
    while !handle.window_should_close() {
        if handle.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = handle.get_mouse_position();

            let col: i32 = (mouse_pos.x as i32) / ui::TILE_SIZE as i32;
            let row: i32 = (mouse_pos.y as i32) / ui::TILE_SIZE as i32;
            let tile: i32 = row * 9 + col;
            println!("click on x{col}y{row}z{tile}");
        }
        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
        rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
        ui::draw_tiles(&mut rldh, grid);
    }
    return;
}
