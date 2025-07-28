use crate::tile::{self, Coord, UnitType};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Grid {
    pub tiles: Vec<tile::Tile>,
    pub blocks: Vec<tile::Unit>,
    pub cols: Vec<tile::Unit>,
    pub rows: Vec<tile::Unit>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new() -> Grid {
        let mut v_tiles: Vec<tile::Tile> = vec![
            tile::Tile {
                val: 0,
                access: tile::Access::Default,
                coord: Coord {
                    w: 0,
                    x: 0,
                    y: 0,
                    z: 0
                },
            };
            81
        ];
        let mut v_blocks: Vec<tile::Unit> = vec![tile::Unit { data: Vec::with_capacity(0), full: 0, count: 0 }; 9];
        let mut v_cols: Vec<tile::Unit> = vec![tile::Unit { data: Vec::with_capacity(0), full: 0, count: 0 }; 9];
        let mut v_rows: Vec<tile::Unit> = vec![tile::Unit { data: Vec::with_capacity(0), full: 0, count: 0 }; 9];
        for i in 0u8..81u8 {
            let index_h: u8 = i % 9;
            let index_v: u8 = i / 9;
            let index_block_x: u8 = (index_h - index_h % 3) / 3;
            let index_block_y: u8 = (index_v - index_v % 3) / 3;
            let index_block: u8 = index_block_y * 3 + index_block_x;
            v_tiles[i as usize].coord.w = index_block;
            v_tiles[i as usize].coord.x = index_h;
            v_tiles[i as usize].coord.y = index_v;
            v_tiles[i as usize].coord.z = i;
            v_blocks[index_block as usize].data.push(i);
            v_cols[index_h as usize].data.push(i);
            v_rows[index_v as usize].data.push(i);
        }
        return Grid {
            tiles: v_tiles,
            blocks: v_blocks,
            cols: v_cols,
            rows: v_rows,
        };
    }

    pub fn get_from(&mut self, loc: &UnitType, (lidx, tidx): (u8, u8)) -> &mut tile::Tile {
        match loc {
            UnitType::Row => {
                return &mut self.tiles[self.rows[lidx as usize].data[tidx as usize] as usize];
            }
            UnitType::Col => {
                return &mut self.tiles[self.cols[lidx as usize].data[tidx as usize] as usize];
            }
            UnitType::Block => {
                return &mut self.tiles[self.blocks[lidx as usize].data[tidx as usize] as usize];
            }
        }
    }

    pub fn get_missing_from(&mut self, loc: &UnitType, n: u8) -> (u16, u8) {
        /*let mut table: Vec<bool> = vec![true; 9];
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
        }*/
        match loc {
            UnitType::Block => {
                return (self.blocks[n as usize].full, self.blocks[n as usize].count);
            }
            UnitType::Col => {
                return (self.cols[n as usize].full, self.cols[n as usize].count);
            }
            UnitType::Row => {
                return (self.rows[n as usize].full, self.rows[n as usize].count);
            }
        }
    }

    pub fn polulate(&mut self, filename: &str) {
        let tiles: Vec<u8> = std::fs::read_to_string(filename)
            .expect("error: cannot read file")
            .into_bytes();
        if tiles.len() != 81 && tiles.len() != 82 {
            panic!("error: the provided sudoku file is invalid");
        }
        for i in 0..tiles.len() { // [ ] add disambiguation for Windows (0x0D0A)
            if tiles[i] > 0x30 && tiles[i] <= 0x39 {
                let index_v: u8 = (i / 9) as u8;
                let index_h: u8 = (i % 9) as u8;
                let index_block_x: u8 = (index_h - index_h % 3) / 3;
                let index_block_y: u8 = (index_v - index_v % 3) / 3;
                let index_block: u8 = index_block_y * 3 + index_block_x;
                let digit: u8 = tiles[i] - 0x30;
                self.tiles[i].val = digit;
                self.tiles[i].access = tile::Access::Default;
                self.blocks[index_block as usize].full |= 1 << (digit - 1);
                self.blocks[index_block as usize].count += 1;
                self.cols[index_h as usize].full |= 1 << (digit - 1);
                self.cols[index_h as usize].count += 1;
                self.rows[index_v as usize].full |= 1 << (digit - 1);
                self.rows[index_v as usize].count += 1;
            } else if tiles[i] == 0x2E {
                self.tiles[i].val = 0;
                self.tiles[i].access = tile::Access::CanEdit;
            }
        }
        println!(
            "blocks: {:?}\ncolumns: {:?}\rows: {:?}",
            self.blocks, self.cols, self.rows
        );
        return;
    }

    pub fn contains(&mut self, loc: UnitType, n: u8, val: u8) -> bool {
        //println!("••• {:?}[{n}] contains {val}: {}", loc, ((self.get_missing_from(&loc, n).0 & (1 << (val - 1))) > 0));
        return val > 0 && ((self.get_missing_from(&loc, n).0 & (1 << (val - 1))) > 0);
    }

    pub fn unit(&mut self, loc: &UnitType) -> &mut Vec<tile::Unit> {
        match loc {
            tile::UnitType::Block => {
                return &mut self.blocks;
            }
            tile::UnitType::Col => {
                return &mut self.cols;
            }
            tile::UnitType::Row => {
                return &mut self.rows;
            }
        }
    }

    pub fn is_full(&mut self) -> bool {
        for i in 0u8..81u8 {
            if self.tiles[i as usize].val == 0 {
                return false;
            }
        }
        return true;
    }
}
