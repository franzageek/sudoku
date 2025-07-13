use std::collections::VecDeque;

use crate::grid;
use crate::tile;
use crate::ui;
use raylib::prelude::RaylibDraw;
use raylib::*;

fn place_couple_candidates(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    n: u8,
    missing: &Vec<u8>,
    empty: &mut VecDeque<tile::Coord>,
    remaining: &Vec<*const Vec<Vec<u8>>>,
    grid: &mut grid::Grid
) {
    for num in 0..missing.len() {
        if empty.len() == 2 {
            unsafe {
                if (!(*remaining[0])[empty[0].x as usize].contains(&missing[num]) 
                    && (*remaining[0])[empty[1].x as usize].contains(&missing[num]))
                    || (!(*remaining[1])[empty[0].y as usize].contains(&missing[num])
                    && (*remaining[1])[empty[1].y as usize].contains(&missing[num])) {
                    grid.get_from(&tile::Unit::Col, (empty[0].x, empty[0].y)).val = missing[num];
                    grid.get_from(&tile::Unit::Col, (empty[0].x, empty[0].y)).access = tile::Access::Step2;
                    println!("//{n} | successfully place number ({}) in cell<{}>", missing[num], empty[0].z);
                    empty.pop_front();
                    {
                        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                        rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
                        ui::draw_tiles(&mut rldh, grid);
                    }
                } else if (!(*remaining[0])[empty[1].x as usize].contains(&missing[num]) 
                && (*remaining[0])[empty[0].x as usize].contains(&missing[num]))
                || (!(*remaining[1])[empty[1].y as usize].contains(&missing[num])
                && (*remaining[1])[empty[0].y as usize].contains(&missing[num])) {
                    grid.get_from(&tile::Unit::Col, (empty[1].x, empty[1].y)).val = missing[num];
                    grid.get_from(&tile::Unit::Col, (empty[1].x, empty[1].y)).access = tile::Access::Step2;
                    println!("//{n} | successfully place number ({}) in cell<{}>", missing[num], empty[1].z);
                    empty.pop_back();
                    {
                        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                        rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
                        ui::draw_tiles(&mut rldh, grid);
                    }
                } else {
                    println!("//! | cannot place number ({}) -- not all conditions are met", missing[num]);
                }// else: not a suitable unit for resolving couples
            } 
        }
        if empty.len() == 1 {
            grid.get_from(&tile::Unit::Col, (empty[0].x, empty[0].y)).val = missing[num];
            grid.get_from(&tile::Unit::Col, (empty[0].x, empty[0].y)).access = tile::Access::Step2;
            println!("//{n} | successfully place number ({}) in cell<{}>", missing[num], empty[0].z);
        } 
    }
}

fn resolve_couples_unit(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    loc: tile::Unit,
    n: u8, 
    grid: &mut grid::Grid
) {
    let missing = grid.get_missing_from(&loc, n);
    if missing.len() == 2 {
        let mut empty: VecDeque<tile::Coord> = VecDeque::with_capacity(0);
        println!("//! | found suitable {} ({n})",
            if loc == tile::Unit::Block {
                "block"
            } else if loc == tile::Unit::Col {
                "column"
            } else {
                "row"
            }
        );
        for cell in 0u8..9u8 {
            let tile: &mut tile::Tile = grid.get_from(&loc, (n, cell));
            if tile.val == 0 {
                empty.push_back(tile.coord);
                println!("//{n} | found free cell ({})", tile.coord.z);
            }
        }
        let mut remaining: Vec<*const Vec<Vec<u8>>> = Vec::with_capacity(0); // cannot reserve directly two elements cause 'temporary value is being dropped'
        if loc == tile::Unit::Block {
            remaining.push(&mut grid.cols);
            remaining.push(&mut grid.rows);
        } else if loc == tile::Unit::Col {
            remaining.push(&mut grid.blocks);
            remaining.push(&mut grid.rows);
        } else {            
            remaining.push(&mut grid.blocks);
            remaining.push(&mut grid.cols);
        }
        place_couple_candidates((handle, thread), n, &missing, &mut empty, &remaining, grid);
    }
}

pub fn resolve_couples(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    grid: &mut grid::Grid,
) {
    for i in 0..9 {
        resolve_couples_unit((handle, thread), tile::Unit::Block, i, grid);
        resolve_couples_unit((handle, thread), tile::Unit::Col, i, grid);
        resolve_couples_unit((handle, thread), tile::Unit::Row, i, grid);
    }
}

/// # Backtracking
/// Solve a sudoku grid by iterating through every possible entry for each cell and keep going until a violation is reached, until the grid is solved
pub fn backtracking(
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
        //std::thread::sleep(std::time::Duration::from_millis(50));
        if grid.tiles[i].val == 0 && grid.tiles[i].access == tile::Access::CanEdit {
            {
                let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
                ui::draw_tiles(&mut rldh, grid);
            }
            let missing_block: Vec<u8> =
                grid.get_missing_from(&tile::Unit::Block, grid.tiles[i].coord.w);
            println!("#{i} | missing from block: {:?}", missing_block);
            let missing_col: Vec<u8> =
                grid.get_missing_from(&tile::Unit::Col, grid.tiles[i].coord.x);
            println!("#{i} | missing from column: {:?}", missing_col);
            let missing_row: Vec<u8> =
                grid.get_missing_from(&tile::Unit::Row, grid.tiles[i].coord.y);
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
                        "<-->| taking a branch on #{i} -- attempt {j} <{}>",
                        possible[j]
                    );
                    if backtracking((&mut handle, &thread), Some(i as u8), grid) {
                        return true;
                    }
                }
            } else {
                println!("<!!>| wrong branch -- backtracking 1 step for #{start}");
                for j in start + 1..=i {
                    if grid.tiles[j].access == tile::Access::CanEdit {
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

/// # Last Remaining Cell
/// Solve a sudoku grid by iterating through the process of determining which blocks are capable of holding a specific number (1..=9) in one cell only.
pub fn lrc(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    rounds: Option<u8>,
    grid: &mut grid::Grid,
) -> bool {
    let mut limit: u8 = 82;
    let mut pass: u8 = 0;
    if let Some(a) = rounds { limit = a }
    while !grid.is_full() && pass < limit {
        pass += 1;
        println!("]{pass}[ | starting round...");
        for num in 1u8..=9u8 {
            println!(">>{num} | starting calculation cycle...");
            for block in 0u8..9u8 {
                if !grid.contains(tile::Unit::Block, block, num) {
                    println!(">>{num} | found suitable block ({block})");
                    let mut possible: Vec<u8> = Vec::with_capacity(0);
                    for cell in 0u8..9u8 {
                        let mut grid_clone: grid::Grid = grid.clone();
                        let tile: &tile::Tile = grid_clone.get_from(
                            &tile::Unit::Block, 
                            (block, cell)
                        );
                        //println!(">>{num} | cell<{cell}>; val<{}>; edit<{}>; x<{}>; y<{}>; colc<{}>; rowc<{}>", tile.val, tile.can_edit as u8, tile.coord.x, tile.coord.y, grid.contains(tile::TileLoc::Col, tile.coord.x, num), grid.contains(tile::TileLoc::Row, tile.coord.y, num));
                        if tile.val == 0 
                            && tile.access == tile::Access::CanEdit
                            && !grid.contains(tile::Unit::Col, tile.coord.x, num) 
                            && !grid.contains(tile::Unit::Row, tile.coord.y, num) {
                            possible.push(cell);
                            println!(">>{num} | found suitable cell in block ({cell})");
                        }
                    } 
                    if possible.len() == 1 {
                        grid.get_from(&tile::Unit::Block, (block, possible[0])).val = num;
                        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                        rldh.clear_background(raylib::color::rcolor(0x00, 0xAA, 0xAA, 0xDD));
                        ui::draw_tiles(&mut rldh, grid);
                        println!("!!{num} | successfully place number!");
                        grid.get_from(&tile::Unit::Block, (block, possible[0])).access = tile::Access::Step1;
                    } else {
                        println!("<!!>| too many cells suitable for <{num}> in block <{block}>");
                    }
                }
            }
        }
    }
    resolve_couples((handle, thread), grid);
    if !grid.is_full() {
        return false;
    } else {
        println!("<!!>| solved in {} passes!", pass);
        return true;
    }
}