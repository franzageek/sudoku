/*
    This is a collection of sudoku solver algorithms.
    Each one is theoretically capable of solving an entire sudoku by itself,
    depending on the difficulty level and whether or not certain conditions are met.
    They are listed here - going from the heaviest to the lightest - under the category they belong to,
    along with a 1 (least efficient) to 5 (most efficient) rating that considers complexity in relation to solving capability:

    [ BRUTEFORCE ]
        - backtracking      [*****] (the only one that can solve any sudoku)

    [ ADAPTIVE ]
        - couple_candidates [**---] (very complex, but can only act on units that lack 2 elements)
        - lrc               [****-] (fairly complex, but extremely efficient; best when combined with `lpn`)
        - lpn               [***--] (very basic, but can cross out quite a lot of cells; best when combined with `lrc`)
*/

use crate::grid;
use crate::tile;
use crate::ui;
use raylib::*;

/* #region Backtracking */
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
            let missing_block: (u16, u8) =
                grid.get_missing_from(&tile::UnitType::Block, grid.tiles[i].coord.w);
            /*println!(
                "#{i} | missing from blk: {:09b},{}",
                (!missing_block.0) & 0x1FF,
                missing_block.1
            );*/
            let missing_col: (u16, u8) =
                grid.get_missing_from(&tile::UnitType::Col, grid.tiles[i].coord.x);
            /*println!(
                "#{i} | missing from col: {:09b},{}",
                (!missing_col.0) & 0x1FF,
                missing_col.1
            );*/
            let missing_row: (u16, u8) =
                grid.get_missing_from(&tile::UnitType::Row, grid.tiles[i].coord.y);
            /*println!(
                "#{i} | missing from row: {:09b},{}",
                (!missing_row.0) & 0x1FF,
                missing_row.1
            );*/
            let mut remaining: Vec<u16> = vec![0; 2];
            let mut biggest_len: u8 = 0;
            let mut biggest: u16 = 0;
            if missing_block.1 > missing_col.1 && missing_block.1 > missing_row.1 {
                biggest_len = missing_block.1;
                remaining[0] = (!missing_col.0) & 0x1FF;
                remaining[1] = (!missing_row.0) & 0x1FF;
                biggest = (!missing_block.0) & 0x1FF;
            }
            if missing_col.1 > biggest_len {
                biggest_len = missing_col.1;
                remaining[0] = (!missing_block.0) & 0x1FF;
                remaining[1] = (!missing_row.0) & 0x1FF;
                biggest = (!missing_col.0) & 0x1FF;
            }
            if missing_row.1 > biggest_len {
                remaining[0] = (!missing_block.0) & 0x1FF;
                remaining[1] = (!missing_col.0) & 0x1FF;
                biggest = (!missing_row.0) & 0x1FF;
            }

            /*println!("#{i} | smallest: {:#b}", biggest);
            println!("#{i} | remaining: {:?}", remaining);*/
            let possible: u16 = (biggest & remaining[0]) & remaining[1];
            //println!("#{i} | possible: {:#b}", possible);
            if possible != 0 && (possible & (possible - 1)) == 0 {
                let val: u8 = (possible.trailing_zeros() + 1) as u8;
                grid.tiles[i].val = val;
                grid.blocks[grid.tiles[i].coord.w as usize].full |= 1 << val - 1;
                grid.blocks[grid.tiles[i].coord.w as usize].count += 1;
                grid.cols[grid.tiles[i].coord.x as usize].full |= 1 << val - 1;
                grid.cols[grid.tiles[i].coord.x as usize].count += 1;
                grid.rows[grid.tiles[i].coord.y as usize].full |= 1 << val - 1;
                grid.rows[grid.tiles[i].coord.y as usize].count += 1;
                println!(
                    "** BCK ** Successfully placed a {} in cell {{ w: {}; x: {}; y: {}; z: {i}}}",
                    val, grid.tiles[i].coord.w, grid.tiles[i].coord.x, grid.tiles[i].coord.y
                );
            } else if possible != 0 && (possible & (possible - 1)) != 0 {
                let mut attempt: u8 = 0;
                for j in 0u8..9u8 {
                    let shamt: u8 = if i % 2 == 0 { 8 - j } else { j };
                    if (possible & (1 << shamt)) > 0 {
                        attempt += 1;
                        grid.tiles[i].val = ((possible & (1 << shamt)).trailing_zeros() + 1) as u8;
                        grid.blocks[grid.tiles[i].coord.w as usize].full |= 1 << shamt;
                        grid.blocks[grid.tiles[i].coord.w as usize].count += 1;
                        grid.cols[grid.tiles[i].coord.x as usize].full |= 1 << shamt;
                        grid.cols[grid.tiles[i].coord.x as usize].count += 1;
                        grid.rows[grid.tiles[i].coord.y as usize].full |= 1 << shamt;
                        grid.rows[grid.tiles[i].coord.y as usize].count += 1;
                        println!(
                            "** BCK ** Taking a branch for cell {{ w: {}; x: {}; y: {}; z: {i}}} -- attempt {attempt} with <{}>",
                            grid.tiles[i].coord.w, grid.tiles[i].coord.x, grid.tiles[i].coord.y, shamt + 1
                        );
                        match backtracking((&mut handle, &thread), Some(i as u8), grid) {
                            None => return None,
                            Some(end) => {
                                for k in i..=end as usize {
                                    if grid.tiles[k].access == tile::Access::CanEdit
                                        && grid.tiles[k].val != 0
                                    {
                                        //println!("empty out: {}", grid.tiles[k].val);
                                        grid.tiles[k].val = 0;
                                        grid.blocks[grid.tiles[k].coord.w as usize]
                                            .update(&mut grid.tiles);
                                        grid.cols[grid.tiles[k].coord.x as usize]
                                            .update(&mut grid.tiles);
                                        grid.rows[grid.tiles[k].coord.y as usize]
                                            .update(&mut grid.tiles);
                                    }
                                }
                            }
                        }
                    }
                }
                return Some(i as u8);
            } else {
                println!(
                    "** BCK ** Wrong branch! Backtracking one step, going back to cell {{ w: {}; x: {}; y: {}; z: {start}}}",
                    grid.tiles[start].coord.w, grid.tiles[start].coord.x, grid.tiles[start].coord.y
                );
                for j in start..i {
                    if grid.tiles[j].access == tile::Access::CanEdit {
                        //println!("empty out: {}", grid.tiles[j].val);
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
/* #endregion */

/* #region Couple Candidates */
/// # Couple Candidates
/// Scan through each unit until one with just two empty cells is found, then check every cell for any violation on one entry so that the cell can be filled in with the other entry, while the other cell can be filled in with this entry.
/// This is an adaptive algorithm that "learns" from previous iterations; hence, a higher number of `rounds` can result in more cells solved.
pub fn couple_candidates(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    rounds: Option<u8>,
    grid: &mut grid::Grid,
) -> bool {
    let mut limit: u8 = 81;
    let mut pass: u8 = 0;
    let mut progress: bool = true;
    if let Some(a) = rounds {
        limit = a
    }
    while !grid.is_full() && pass < limit {
        if !progress {
            return false;
        }
        pass += 1;
        progress = false;
        println!("** CC ** Starting round {pass:02}");
        for i in 0u8..9u8 {
            //println!(")){i} | scanning units...");
            progress |= resolve_unit_couples((handle, thread), tile::UnitType::Block, i, grid)
                | resolve_unit_couples((handle, thread), tile::UnitType::Col, i, grid)
                | resolve_unit_couples((handle, thread), tile::UnitType::Row, i, grid);
            // leave `progress` untouched until some progress is made
        }
    }
    if grid.is_full() {
        println!("!! CC !! Solved in {pass} passes");
        return true;
    } else {
        return false;
    }
}

fn resolve_unit_couples(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    loc: tile::UnitType,
    n: u8,
    grid: &mut grid::Grid,
) -> bool {
    let mut unit: [tile::UnitType; 3] = [tile::UnitType::Block; 3];
    let mut possible: (tile::Coord, tile::Coord) = (tile::Coord::zeroed(), tile::Coord::zeroed());
    let mut coord: [(u8, u8); 3] = [(0, 0); 3];
    let mut candidates_bitmask: u16 = 0;
    let mut done: bool = false;
    /*println!(
        "))((|size is {} for {:?}[{n}]",
        grid.unit(&loc)[n as usize].count,
        &loc
    );*/
    match loc {
        tile::UnitType::Block => {
            if grid.blocks[n as usize].count == 7 {
                candidates_bitmask = !grid.blocks[n as usize].full & 0x1FF;
                let mut possible_bitmask: u16 = 0;
                for i in 0u8..9u8 {
                    if grid.get_from(&tile::UnitType::Block, (n, i)).val == 0 {
                        possible_bitmask |= 1 << i;
                    }
                }
                unit = [
                    tile::UnitType::Block,
                    tile::UnitType::Col,
                    tile::UnitType::Row,
                ];
                possible = (
                    grid.get_from(&loc, (n, possible_bitmask.trailing_zeros() as u8))
                        .coord,
                    grid.get_from(
                        &loc,
                        (n, (9 - 1) - (possible_bitmask.leading_zeros() - 7) as u8),
                    )
                    .coord,
                );
                coord = [
                    (possible.0.w, possible.1.w),
                    (possible.0.x, possible.1.x),
                    (possible.0.y, possible.1.y),
                ];
                done = true;
            }
        }
        tile::UnitType::Col => {
            if grid.cols[n as usize].count == 7 {
                candidates_bitmask = !grid.cols[n as usize].full & 0x1FF;
                let mut possible_bitmask: u16 = 0;
                for i in 0u8..9u8 {
                    if grid.get_from(&tile::UnitType::Col, (n, i)).val == 0 {
                        possible_bitmask |= 1 << i;
                    }
                }
                unit = [
                    tile::UnitType::Col,
                    tile::UnitType::Block,
                    tile::UnitType::Row,
                ];
                possible = (
                    grid.get_from(&loc, (n, possible_bitmask.trailing_zeros() as u8))
                        .coord,
                    grid.get_from(
                        &loc,
                        (n, (9 - 1) - (possible_bitmask.leading_zeros() - 7) as u8),
                    )
                    .coord,
                );
                coord = [
                    (possible.0.x, possible.1.x),
                    (possible.0.w, possible.1.w),
                    (possible.0.y, possible.1.y),
                ];
                done = true;
            }
        }
        tile::UnitType::Row => {
            if grid.rows[n as usize].count == 7 {
                candidates_bitmask = !grid.rows[n as usize].full & 0x1FF;
                let mut possible_bitmask: u16 = 0;
                for i in 0u8..9u8 {
                    if grid.get_from(&tile::UnitType::Row, (n, i)).val == 0 {
                        possible_bitmask |= 1 << i;
                    }
                }
                unit = [
                    tile::UnitType::Row,
                    tile::UnitType::Block,
                    tile::UnitType::Col,
                ];
                possible = (
                    grid.get_from(&loc, (n, possible_bitmask.trailing_zeros() as u8))
                        .coord,
                    grid.get_from(
                        &loc,
                        (n, (9 - 1) - (possible_bitmask.leading_zeros() - 7) as u8),
                    )
                    .coord,
                );
                coord = [
                    (possible.0.y, possible.1.y),
                    (possible.0.w, possible.1.w),
                    (possible.0.x, possible.1.x),
                ];
                done = true;
            }
        }
    }
    if done {
        /*println!("coord: {:?}", coord);
        println!(
            "candidates: {:?}",
            (
                (candidates_bitmask.trailing_zeros() + 1) as u8,
                9 - (candidates_bitmask.leading_zeros() - 7) as u8
            )
        );*/
        return resolve_couple_candidates(
            (handle, thread),
            unit,
            coord,
            (possible.0.z, possible.1.z),
            (
                (candidates_bitmask.trailing_zeros() + 1) as u8,
                9 - (candidates_bitmask.leading_zeros() - 7) as u8,
            ),
            grid,
        );
    }
    return false;
}

fn resolve_couple_candidates(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    unit: [tile::UnitType; 3],
    coord: [(u8, u8); 3],
    zcoord: (u8, u8),
    mut candidates: (u8, u8),
    grid: &mut grid::Grid,
) -> bool {
    let mut done: bool = false;
    if (
        // check if placing candidate<0> in possible_cell[1] would result in a violation (not permitted, <0> must be placed in [0] and <1> in [1])
        grid.unit(&unit[1])[coord[1].1 as usize].full & (1 << candidates.0 - 1) > 0
            || grid.unit(&unit[2])[coord[2].1 as usize].full & (1 << candidates.0 - 1) > 0
    ) || (
        // check if placing candidate<1> in possible_cell[0] would result in a violation (not permitted, <0> must be placed in [0] and <1> in [1])
        grid.unit(&unit[1])[coord[1].0 as usize].full & (1 << candidates.1 - 1) > 0
            || grid.unit(&unit[2])[coord[2].0 as usize].full & (1 << candidates.1 - 1) > 0
        // NOTE: checking if the candidate is missing from other units of the other possible_cell is pointless,
        //       since there must be at least one place where to put each candidate,
        //       so we expect AT LEAST a cell from whose units the candidate is missing,
        //       and the condition would always be true
    ) {
        /*println!(
            "))((| found ({}) in [{:?}||{:?}]<{}||{}>::<{}>, applying <{}> to <{}> and <{}> to <{}>",
            candidates.0,
            unit[1],
            unit[2],
            coord[1].0,
            coord[2].0,
            zcoord.0,
            candidates.1,
            zcoord.1,
            candidates.0,
            zcoord.0
        );*/
        grid.tiles[zcoord.0 as usize].val = candidates.0;
        grid.tiles[zcoord.1 as usize].val = candidates.1;
        println!(
            "** CC ** Successfully placed a {} in cell {{w: {}; x: {}; y: {}; z: {}}} and a {} in cell {{w: {}; x: {}; y: {}; z: {}}}",
            candidates.0,
            grid.tiles[zcoord.0 as usize].coord.w,
            grid.tiles[zcoord.0 as usize].coord.x,
            grid.tiles[zcoord.0 as usize].coord.y,
            zcoord.0,
            candidates.1,
            grid.tiles[zcoord.1 as usize].coord.w,
            grid.tiles[zcoord.1 as usize].coord.x,
            grid.tiles[zcoord.1 as usize].coord.y,
            zcoord.1
        );
        done = true;
    } else if (
        // check if placing candidate<1> in possible_cell[1] would result in a violation (not permitted, <1> must be placed in [0] and <0> in [1])
        grid.unit(&unit[1])[coord[1].1 as usize].full & (1 << candidates.1 - 1) > 0
            || grid.unit(&unit[2])[coord[2].1 as usize].full & (1 << candidates.1 - 1) > 0
    ) || (
        // check if placing candidate<0> in possible_cell[0] would result in a violation (not permitted, <1> must be placed in [0] and <0> in [1])
        grid.unit(&unit[1])[coord[1].0 as usize].full & (1 << candidates.0 - 1) > 0
            || grid.unit(&unit[2])[coord[2].0 as usize].full & (1 << candidates.0 - 1) > 0
    ) {
        grid.tiles[zcoord.0 as usize].val = candidates.1;
        grid.tiles[zcoord.1 as usize].val = candidates.0;
        println!(
            "** CC ** Successfully placed a {} in cell {{w: {}; x: {}; y: {}; z: {}}} and a {} in cell {{w: {}; x: {}; y: {}; z: {}}}",
            candidates.1,
            grid.tiles[zcoord.0 as usize].coord.w,
            grid.tiles[zcoord.0 as usize].coord.x,
            grid.tiles[zcoord.0 as usize].coord.y,
            zcoord.0,
            candidates.0,
            grid.tiles[zcoord.1 as usize].coord.w,
            grid.tiles[zcoord.1 as usize].coord.x,
            grid.tiles[zcoord.1 as usize].coord.y,
            zcoord.1
        );
        std::mem::swap(&mut candidates.0, &mut candidates.1);
        done = true;
    }
    if done {
        grid.tiles[zcoord.0 as usize].access = tile::Access::CouplePass;
        grid.tiles[zcoord.1 as usize].access = tile::Access::CouplePass;

        grid.unit(&unit[0])[coord[0].0 as usize].count = 9;
        grid.unit(&unit[0])[coord[0].0 as usize].full = 0x1FF;

        grid.unit(&unit[1])[coord[1].0 as usize].full |= 1 << candidates.0 - 1;
        grid.unit(&unit[1])[coord[1].0 as usize].count += 1;
        grid.unit(&unit[1])[coord[1].1 as usize].full |= 1 << candidates.1 - 1;
        grid.unit(&unit[1])[coord[1].1 as usize].count += 1;

        grid.unit(&unit[2])[coord[2].0 as usize].full |= 1 << candidates.0 - 1;
        grid.unit(&unit[2])[coord[2].0 as usize].count += 1;
        grid.unit(&unit[2])[coord[2].1 as usize].full |= 1 << candidates.1 - 1;
        grid.unit(&unit[2])[coord[2].1 as usize].count += 1;

        let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
        ui::draw_tiles(&mut rldh, grid);
        return true;
    }
    println!("** CC ** Unable to fill out any cell in current unit");
    return false;
}
/* #endregion */

/* #region Last Remaining Cell */
/// # Last Remaining Cell
/// Solve a sudoku grid by iterating through the process of determining which blocks are capable of holding a specific number (1..=9) in one cell only.
/// This is an adaptive algorithm that "learns" from previous iterations; hence, a higher number of `rounds` can result in more cells solved.
pub fn lrc(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    rounds: Option<u8>,
    grid: &mut grid::Grid,
) -> bool {
    let mut limit: u8 = 81;
    let mut pass: u8 = 0;
    let mut progress: bool = true;
    if let Some(a) = rounds {
        limit = a
    }
    while !grid.is_full() && pass < limit {
        if !progress {
            return false;
        }
        if lpn((handle, thread), rounds, grid) {
            println!("!! LRC !! Solved in {pass} passes");
            return true;
        };
        pass += 1;
        progress = false;
        println!("** LRC ** Starting round {pass:02}");
        for num in 1u8..=9u8 {
            println!("** LRC ** Starting cycle for number {num}");
            for block in 0u8..9u8 {
                if !grid.contains(tile::UnitType::Block, block, num) {
                    //println!(">>{num} | found suitable block ({block})");
                    let mut possible: u16 = 0; /*Vec<u8> = Vec::with_capacity(0);*/
                    for cell in 0u8..9u8 {
                        let tile: &tile::Tile =
                            grid.get_from(&tile::UnitType::Block, (block, cell));
                        let x: u8 = tile.coord.x;
                        let y: u8 = tile.coord.y;
                        //println!(">>{num} | cell<{cell}>; val<{}>; edit<{}>; x<{}>; y<{}>; colc<{}>; rowc<{}>", tile.val, tile.access as u8, tile.coord.x, tile.coord.y, grid.contains(tile::UnitType::Col, tile.coord.x, num), grid.contains(tile::UnitType::Row, tile.coord.y, num));
                        if tile.val == 0
                            && tile.access == tile::Access::CanEdit
                            && !grid.contains(tile::UnitType::Col, x, num)
                            && !grid.contains(tile::UnitType::Row, y, num)
                        {
                            possible |= 1 << (cell);
                            //println!(">>{num} | found suitable cell in block ({cell})");
                        }
                    }
                    if possible > 0 && (possible & (possible - 1)) == 0 {
                        let tile: &mut tile::Tile = grid.get_from(
                            &tile::UnitType::Block,
                            (block, possible.trailing_zeros() as u8),
                        );
                        tile.val = num;
                        tile.access = tile::Access::LRCPass;
                        let coord: tile::Coord = tile.coord;
                        grid.blocks[block as usize].full |= 1 << (num - 1);
                        grid.blocks[block as usize].count += 1;
                        grid.cols[coord.x as usize].full |= 1 << (num - 1);
                        grid.cols[coord.x as usize].count += 1;
                        grid.rows[coord.y as usize].full |= 1 << (num - 1);
                        grid.rows[coord.y as usize].count += 1;
                        progress = true;
                        println!("** LRC ** Successfully placed a {num} in cell {{w: {block}; x: {}; y: {}; z: {}}}", coord.x, coord.y, coord.z);
                        //println!("!!{num} | successfully place number!");
                        let mut rldh: core::drawing::RaylibDrawHandle =
                            handle.begin_drawing(&thread);
                        ui::draw_tiles(&mut rldh, grid);
                    } else {
                        println!("** LRC ** Too many options for putting a {num} in block {block}");
                    }
                }
            }
        }
    }
    //resolve_couples((handle, thread), grid);
    if !grid.is_full() {
        if !lpn((handle, thread), None, grid) // one last round of LPN to see if there are any leftovers
        && !couple_candidates((handle, thread), None, grid) { // one last round of CC to see if there are any leftovers
            return false;
        }
    }
    println!("!! LRC !! Solved in {pass} passes");
    return true;
}
/* #endregion */

/* #region Last Possible Number */
/// # Last Possible Number
/// Solve a sudoku grid by iterating through each cell and filling those with one possible candidate only right away.
/// This is an adaptive algorithm that "learns" from previous iterations; hence, a higher number of `rounds` can result in more cells solved.
pub fn lpn(
    (handle, thread): (&mut RaylibHandle, &RaylibThread),
    rounds: Option<u8>,
    grid: &mut grid::Grid,
) -> bool {
    let mut limit: u8 = 81;
    let mut pass: u8 = 0;
    let mut progress: bool = true;
    if let Some(a) = rounds {
        limit = a
    }
    while !grid.is_full() && pass < limit {
        if !progress {
            return false;
        }
        pass += 1;
        progress = false;
        println!("** LPN ** Starting round {pass:02}");
        for i in 0u8..81u8 {
            if grid.tiles[i as usize].access == tile::Access::CanEdit {
                let coord: tile::Coord = grid.tiles[i as usize].coord;
                let (block_missing, _) = grid.get_missing_from(&tile::UnitType::Block, coord.w);
                let (col_missing, _) = grid.get_missing_from(&tile::UnitType::Col, coord.x);
                let (row_missing, _) = grid.get_missing_from(&tile::UnitType::Row, coord.y);
                let possible: u16 = !((block_missing | col_missing) | row_missing) & 0x1FF;
                if possible > 0 && (possible & (possible - 1)) == 0 {
                    let val: u8 = (possible.trailing_zeros() + 1) as u8;
                    grid.tiles[i as usize].val = val;
                    grid.tiles[i as usize].access = tile::Access::LPNPass;
                    grid.blocks[coord.w as usize].full |= 1 << val - 1;
                    grid.blocks[coord.w as usize].count += 1;
                    grid.cols[coord.x as usize].full |= 1 << val - 1;
                    grid.cols[coord.x as usize].count += 1;
                    grid.rows[coord.y as usize].full |= 1 << val - 1;
                    grid.rows[coord.y as usize].count += 1;
                    progress = true;
                    println!("** LRC ** Successfully placed a {val} in cell {{w: {}; x: {}; y: {}; z: {}}}", coord.w, coord.x, coord.y, coord.z);
                    let mut rldh: core::drawing::RaylibDrawHandle = handle.begin_drawing(&thread);
                    ui::draw_tiles(&mut rldh, grid);
                }
            }
        }
    }
    if !grid.is_full() {
        return false;
    } else {
        println!("!! LPN !! Solved in {pass} passes");
        return true;
    }
}
/* #endregion */
