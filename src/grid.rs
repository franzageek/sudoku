use crate::tile;

#[allow(dead_code)]
pub struct Grid {
    pub tiles: Vec<tile::Tile>,
    pub rows: Vec<Vec<u8>>,
    pub cols: Vec<Vec<u8>>,
    pub blocks: Vec<Vec<u8>>,
}

#[allow(dead_code)]
pub enum TileLoc {
    Row,
    Col,
    Block,
}

#[allow(dead_code)]
impl Grid {
    pub fn new() -> Grid {
        let tiles: Vec<tile::Tile> = vec![
            tile::Tile {
                val: 0,
                can_edit: false
            };
            81
        ];
        let mut v_rows: Vec<Vec<u8>> = vec![Vec::with_capacity(0) as Vec<u8>; 9];
        let mut v_cols: Vec<Vec<u8>> = vec![Vec::with_capacity(0) as Vec<u8>; 9];
        let mut v_blocks: Vec<Vec<u8>> = vec![Vec::with_capacity(0) as Vec<u8>; 9];
        for i in 0..81 {
            let index_v: u8 = i / 9;
            let index_h: u8 = i % 9;
            let index_block_x: u8 = (index_h - index_h % 3) / 3;
            let index_block_y: u8 = (index_v - index_v % 3) / 3;
            let index_block: u8 = index_block_y * 3 + index_block_x;
            v_rows[index_v as usize].push(i);
            v_cols[index_h as usize].push(i);
            v_blocks[index_block as usize].push(i);
        }
        println!(
            "rows: {:?}\ncolumns: {:?}\nblocks: {:?}",
            v_rows, v_cols, v_blocks
        );
        return Grid {
            tiles: tiles,
            rows: v_rows,
            cols: v_cols,
            blocks: v_blocks,
        };
    }
    pub fn get_from(&mut self, loc: &TileLoc, (lidx, tidx): (u8, u8)) -> &tile::Tile {
        match loc {
            TileLoc::Row => {
                return &self.tiles[self.rows[lidx as usize][tidx as usize] as usize];
            }
            TileLoc::Col => {
                return &self.tiles[self.cols[lidx as usize][tidx as usize] as usize];
            }
            TileLoc::Block => {
                return &self.tiles[self.blocks[lidx as usize][tidx as usize] as usize];
            }
        }
    }
    pub fn get_missing_from(&mut self, loc: TileLoc, n: u8) -> Vec<u8> {
        let mut table: Vec<bool> = vec![true; 9];
        let mut missing: Vec<u8> = Vec::with_capacity(0);
        for i in 0usize..9usize {
            if self.get_from(&loc, (n, i as u8)).val != 0 {
                table[(self.get_from(&loc, (n, i as u8)).val - 1) as usize] = false;
            }
        }
        for i in 0usize..9usize {
            if table[i] {
                missing.push((i + 1) as u8);
            }
        }
        return missing;
    }

    pub fn polulate(&mut self, filename: &str) {
        let tiles: Vec<u8> = std::fs::read_to_string(filename)
            .expect("error: cannot read file")
            .into_bytes();
        for i in 0..tiles.len() - 1 {
            self.tiles[i] = tile::Tile {
                val: tiles[i] - 0x30,
                can_edit: false,
            };
        }
        return;
    }
}
