use crate::grid;
use crate::tile;

pub fn solve(grid: &mut grid::Grid) {
    for i in 0..81 {
        if grid.tiles[i].val == 0 && grid.tiles[i].can_edit {
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
            if possible.len() == 1 {
                grid.tiles[i].val = possible[0];
                println!("#{i} | >> came up with {} <<", possible[0]);
            } else {
                println!(
                    "too many possibilities for tile #{i}: {:?}, skipping",
                    possible
                );
            }
        }
    }
}
