use crate::grid;
use crate::tile;
use crate::ui;
use raylib::*;

/*fn place_couple_candidates(
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
                    && (*remaining[0])[empty[1].x as usize].contains(&missing[num])) //FIXME: based on the false assumprion that remaining[0] is always ‘cols‘ and [1] is ‘rows‘
                    || (!(*remaining[1])[empty[0].y as usize].contains(&missing[num])
                    && (*remaining[1])[empty[1].y as usize].contains(&missing[num])) {
                    grid.get_from(&tile::UnitType::Col, (empty[0].x, empty[0].y)).val = missing[num];
                    grid.get_from(&tile::UnitType::Col, (empty[0].x, empty[0].y)).access = tile::Access::Step2;
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
                    grid.get_from(&tile::UnitType::Col, (empty[1].x, empty[1].y)).val = missing[num];
                    grid.get_from(&tile::UnitType::Col, (empty[1].x, empty[1].y)).access = tile::Access::Step2;
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
            grid.get_from(&tile::UnitType::Col, (empty[0].x, empty[0].y)).val = missing[num];
            grid.get_from(&tile::UnitType::Col, (empty[0].x, empty[0].y)).access = tile::Access::Step2;
            println!("//{n} | successfully place number ({}) in cell<{}>", missing[num], empty[0].z);
        } 
    }
}

fn resolve_couples_unit(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    loc: tile::UnitType,
    n: u8, 
    grid: &mut grid::Grid
) {
    let missing: (u16, u8) = grid.get_missing_from(&loc, n);
    if missing.1 == 2 {
        let mut empty: VecDeque<tile::Coord> = VecDeque::with_capacity(0);
        println!("//! | found suitable {} ({n})",
            if loc == tile::UnitType::Block {
                "block"
            } else if loc == tile::UnitType::Col {
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
        if loc == tile::UnitType::Block {
            remaining.push(&mut grid.cols);
            remaining.push(&mut grid.rows);
        } else if loc == tile::UnitType::Col {
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
        resolve_couples_unit((handle, thread), tile::UnitType::Block, i, grid);
        resolve_couples_unit((handle, thread), tile::UnitType::Col, i, grid);
        resolve_couples_unit((handle, thread), tile::UnitType::Row, i, grid);
    }
}*/

/// # Backtracking
/// Solve a sudoku grid by iterating through every possible entry for each cell and keep going until a violation is reached, until the grid is solved
pub fn backtracking(
    (mut handle, thread): (&mut RaylibHandle, &RaylibThread),
    tile: Option<u8>,
    grid: &mut grid::Grid,
) -> Option<u8> {
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
                ui::draw_tiles(&mut rldh, grid);
            }
            //std::io::stdin().read_line(&mut String::new()).unwrap();

            /*for j in 1u8..=9u8 {
                grid.tiles[i].val = j;
                if !game::check_violation(i as u8, grid) {
                    if backtracking((&mut handle, &thread), Some(i as u8), grid) {
                        return true;
                    }
                }
            }*/
            let missing_block: (u16, u8) = grid.get_missing_from(&tile::UnitType::Block, grid.tiles[i].coord.w);
            println!("#{i} | missing from blk: {:09b},{}", (!missing_block.0) & 0x1FF, missing_block.1);
            let missing_col: (u16, u8) = grid.get_missing_from(&tile::UnitType::Col, grid.tiles[i].coord.x);
            println!("#{i} | missing from col: {:09b},{}", (!missing_col.0) & 0x1FF, missing_col.1);
            let missing_row: (u16, u8) = grid.get_missing_from(&tile::UnitType::Row, grid.tiles[i].coord.y);
            println!("#{i} | missing from row: {:09b},{}", (!missing_row.0) & 0x1FF, missing_row.1);
            let mut remaining: Vec<u16> = vec![0; 2];
            let mut biggest_len: u8 = 0;
            let mut biggest: u16 = 0;
            if missing_block.1 > missing_col.1
                && missing_block.1 > missing_row.1
            {
                biggest_len = missing_block .1;
                remaining[0] = (!missing_col.0) & 0x1FF;
                remaining[1] = (!missing_row.0) & 0x1FF;
                biggest = (!missing_block.0) & 0x1FF;
            }
            if missing_col.1 > biggest_len
            {
                biggest_len = missing_col.1;
                remaining[0] = (!missing_block.0) & 0x1FF;
                remaining[1] = (!missing_row.0) & 0x1FF;
                biggest = (!missing_col.0) & 0x1FF;
            }
            if missing_row.1 > biggest_len
            {
                remaining[0] = (!missing_block.0) & 0x1FF;
                remaining[1] = (!missing_col.0) & 0x1FF;
                biggest = (!missing_row.0) & 0x1FF;
            }

            println!("#{i} | smallest: {:#b}", biggest);
            println!("#{i} | remaining: {:?}", remaining);
            let possible: u16 = (biggest & remaining[0]) & remaining[1];
            println!("#{i} | possible: {:#b}", possible);
            if possible != 0 && (possible & (possible - 1)) == 0 {
                let val: u8 = (possible.trailing_zeros() + 1) as u8;
                grid.tiles[i].val = val;
                grid.blocks[grid.tiles[i].coord.w as usize].full |= 1 << val-1;
                grid.blocks[grid.tiles[i].coord.w as usize].count += 1;
                grid.cols[grid.tiles[i].coord.x as usize].full |= 1 << val-1;
                grid.cols[grid.tiles[i].coord.x as usize].count += 1;
                grid.rows[grid.tiles[i].coord.y as usize].full |= 1 << val-1;
                grid.rows[grid.tiles[i].coord.y as usize].count += 1;
                println!("#{i} | >> came up with {} (w: {}; x: {}; y: {}) <<", val, grid.tiles[i].coord.w, grid.tiles[i].coord.x, grid.tiles[i].coord.y);
            } else if possible != 0 && (possible & (possible - 1)) != 0 {
                let mut attempt: u8 = 0;
                for j in 0u8..9u8 { 
                    if (possible & (1 << j)) > 0 {
                        attempt += 1;
                        grid.tiles[i].val = ((possible & (1 << j)).trailing_zeros() + 1) as u8;
                        grid.blocks[grid.tiles[i].coord.w as usize].full |= 1 << j;
                        grid.blocks[grid.tiles[i].coord.w as usize].count += 1;
                        grid.cols[grid.tiles[i].coord.x as usize].full |= 1 << j;
                        grid.cols[grid.tiles[i].coord.x as usize].count += 1;
                        grid.rows[grid.tiles[i].coord.y as usize].full |= 1 << j;
                        grid.rows[grid.tiles[i].coord.y as usize].count += 1;
                        println!(
                            "<-->| taking a branch on #{i}{{w: {}; x: {}; y: {}}} -- attempt {attempt} <{}>",
                            grid.tiles[i].coord.w, grid.tiles[i].coord.x, grid.tiles[i].coord.y,
                            j+1
                        );
                        match backtracking((&mut handle, &thread), Some(i as u8), grid) {
                            None => return None,
                            Some(end) => {
                                for k in i..=end as usize {
                                    if grid.tiles[k].access == tile::Access::CanEdit && grid.tiles[k].val != 0{
                                        println!("empty out: {}", grid.tiles[k].val);
                                        grid.tiles[k].val = 0;
                                        grid.blocks[grid.tiles[k].coord.w as usize].update(&mut grid.tiles);
                                        grid.cols[grid.tiles[k].coord.x as usize].update(&mut grid.tiles);
                                        grid.rows[grid.tiles[k].coord.y as usize].update(&mut grid.tiles);
                                    }
                                }
                            }
                        }
                    }
                }
                return Some(i as u8);
            } else {
                println!("<!!>| wrong branch -- backtracking 1 step for #{start}");
                for j in start..i {
                    if grid.tiles[j].access == tile::Access::CanEdit {
                        println!("empty out: {}", grid.tiles[j].val);
                        grid.tiles[j].val = 0;
                        grid.blocks[grid.tiles[j].coord.w as usize].update(&mut grid.tiles);
                        grid.cols[grid.tiles[j].coord.x as usize].update(&mut grid.tiles);
                        grid.rows[grid.tiles[j].coord.y as usize].update(&mut grid.tiles);
                    }
                }
                return Some(start as u8);
            }
        }
    }
    let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
    ui::draw_tiles(&mut rldh, grid);
    return None; // ‘None‘ means success
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
                if !grid.contains(tile::UnitType::Block, block, num) {
                    println!(">>{num} | found suitable block ({block})");
                    let mut possible: u16 = 0;/*Vec<u8> = Vec::with_capacity(0);*/
                    for cell in 0u8..9u8 {
                        let tile: &tile::Tile = grid.get_from(
                            &tile::UnitType::Block, 
                            (block, cell)
                        );
                        let x: u8 = tile.coord.x;
                        let y: u8 = tile.coord.y;
                        //println!(">>{num} | cell<{cell}>; val<{}>; edit<{}>; x<{}>; y<{}>; colc<{}>; rowc<{}>", tile.val, tile.access as u8, tile.coord.x, tile.coord.y, grid.contains(tile::UnitType::Col, tile.coord.x, num), grid.contains(tile::UnitType::Row, tile.coord.y, num));
                        if tile.val == 0 
                            && tile.access == tile::Access::CanEdit
                            && !grid.contains(tile::UnitType::Col, x, num) 
                            && !grid.contains(tile::UnitType::Row, y, num) {
                            possible |= 1 << (cell);
                            //println!(">>{num} | found suitable cell in block ({cell})");
                        }
                    } 
                    if (possible & possible-1)  == 0 {
                        let tile: &mut tile::Tile = grid.get_from(&tile::UnitType::Block, (block, possible.trailing_zeros() as u8));
                        tile.val = num;
                        tile.access = tile::Access::Step1;
                        let coord: tile::Coord = tile.coord;
                        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                        ui::draw_tiles(&mut rldh, grid);
                        println!("!!{num} | successfully place number!");
                        grid.blocks[block as usize].full |= 1 << (num - 1);
                        grid.blocks[block as usize].count += 1;
                        grid.cols[coord.x as usize].full |= 1 << (num - 1);
                        grid.cols[coord.x as usize].count += 1;
                        grid.rows[coord.y as usize].full |= 1 << (num - 1); // set local ¡tile¡ variable
                        grid.rows[coord.y as usize].count += 1; // set local ¡tile¡ variable
                    } else {
                        println!("<!!>| too many cells suitable for <{num}> in block <{block}>");
                    }
                }
            }
        }
    }
    //resolve_couples((handle, thread), grid);
    if !grid.is_full() {
        return false;
    } else {
        println!("<!!>| solved in {} passes!", pass);
        return true;
    }
}