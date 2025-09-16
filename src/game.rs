use crate::grid;
use crate::solver;
use crate::tile;
use crate::ui;
use raylib::prelude::RaylibDraw;
use raylib::*;

#[allow(dead_code)]
pub fn check_violation(n: u8, grid: &mut grid::Grid) -> bool {
    let w: u8 = grid.tiles[n as usize].coord.w;
    let x: u8 = grid.tiles[n as usize].coord.x;
    let y: u8 = grid.tiles[n as usize].coord.y;
    let val: u8 = grid.tiles[n as usize].val;
    if grid.contains(tile::UnitType::Block, w, val)
        || grid.contains(tile::UnitType::Col, x, val)
        || grid.contains(tile::UnitType::Row, y, val)
    {
        return false;
    }
    return true;
}

#[allow(dead_code)]
pub fn main_loop((mut handle, thread): (RaylibHandle, RaylibThread), grid: &mut grid::Grid) {
    while !handle.window_should_close() {
        if handle.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = handle.get_mouse_position();

            let col: i32 = (mouse_pos.x as i32) / ui::TILE_SIZE as i32;
            let row: i32 = (mouse_pos.y as i32) / ui::TILE_SIZE as i32;
            let tile: i32 = row * 9 + col;
            /* vvvv DISABLE vvvv */
            assert_eq!(col as u8, grid.tiles[tile as usize].coord.x);
            assert_eq!(row as u8, grid.tiles[tile as usize].coord.y);
            assert_eq!(tile as u8, grid.tiles[tile as usize].coord.z);
            /* ^^^^ DISABLE ^^^^ */
            println!("click on x{col}y{row}z{tile}");
        }
        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
        rldh.clear_background(raylib::color::rcolor(0, 0, 0, 0xFF));
        ui::draw_tiles(&mut rldh, grid);
    }
    return;
}

#[allow(unreachable_code)]
pub fn solve((mut handle, thread): (RaylibHandle, RaylibThread), grid: &mut grid::Grid) {
    //if solver::backtracking((&mut handle, &thread), None, grid) {
    ui::draw_tiles(&mut handle.begin_drawing(&thread), grid);
    if solver::lrc((&mut handle, &thread), None, grid) && grid.is_full() {
        println!("< !! > Solved!");
    } else {
        ui::draw_tiles(&mut handle.begin_drawing(&thread), grid);
        println!("< !! > Cannot solve sudoku with current method alone -- starting backtracker for the final step");
        if solver::backtracking((&mut handle, &thread), None, grid).is_none() {
            println!("< !! > Solved!");
        } else {
            eprintln!("error: cannot solve sudoku");
        }
    }
    while !handle.window_should_close() {
        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
        ui::draw_tiles(&mut rldh, grid);
        rldh.clear_background(raylib::color::rcolor(0, 0, 0, 0xFF));
    }
    return;
}
