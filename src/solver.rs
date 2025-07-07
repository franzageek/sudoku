use crate::grid;
use crate::tile;
use crate::ui;
use raylib::prelude::RaylibDraw;
use raylib::*;

pub fn solve(
    (mut handle, thread): (&mut RaylibHandle, &RaylibThread),
    tile: Option<u8>,
    grid: &mut grid::Grid,
) -> bool {
    let start: usize;
    match tile {
        Some(a) => start = a as usize,
        None => start = 0,
    }
    for i in start..81 {
        //sleep(Duration::from_millis(1));
        if grid.tiles[i].val == 0 && grid.tiles[i].can_edit {
            {
                let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
                ui::draw_tiles(&mut rldh, grid);
            }
            let missing_block: Vec<u8> =
                grid.get_missing_from(tile::TileLoc::Block, grid.tiles[i].coord.w);
            println!("#{i} | missing from block: {:?}", missing_block);
            let missing_col: Vec<u8> =
                grid.get_missing_from(tile::TileLoc::Col, grid.tiles[i].coord.x);
            println!("#{i} | missing from column: {:?}", missing_col);
            let missing_row: Vec<u8> =
                grid.get_missing_from(tile::TileLoc::Row, grid.tiles[i].coord.y);
            println!("#{i} | missing from row: {:?}", missing_row);
            let mut possible: Vec<u8> = Vec::with_capacity(0);
            let mut remaining: Vec<Vec<u8>> = vec![Vec::with_capacity(0); 2];
            let smallest: Vec<u8> = if missing_block.len() < missing_col.len()
                && missing_block.len() < missing_row.len()
            {
                remaining[0] = missing_col;
                remaining[1] = missing_row;
                missing_block
            } else if missing_col.len() < missing_block.len()
                && missing_col.len() < missing_row.len()
            {
                remaining[0] = missing_block;
                remaining[1] = missing_row;
                missing_col
            } else {
                remaining[0] = missing_block;
                remaining[1] = missing_col;
                missing_row
            };
            println!("#{i} | smallest: {:?}", smallest);
            println!("#{i} | remaining: {:?}", remaining);
            for j in smallest {
                if remaining[0].contains(&j) && remaining[1].contains(&j) {
                    possible.push(j);
                }
            }
            println!("#{i} | possible: {:?}", possible);
            if possible.len() == 1 {
                grid.tiles[i].val = possible[0];
                println!("#{i} | >> came up with {} <<", possible[0]);
            } else if possible.len() > 1 {
                for j in 0..possible.len() {
                    grid.tiles[i].val = possible[j];
                    println!(
                        "<--> taking a branch on #{i} -- attempt {j} <{}>",
                        possible[j]
                    );
                    if solve((&mut handle, &thread), Some(i as u8), grid) {
                        return true;
                    }
                }
            } else {
                println!("<!!> wrong branch -- backtracking 1 step for #{start}");
                for j in start + 1..=i {
                    if grid.tiles[j].can_edit {
                        grid.tiles[j].val = 0;
                    }
                }
                return false;
            }
        }
    }
    let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
    rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
    ui::draw_tiles(&mut rldh, grid);

    return true;
}
