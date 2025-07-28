use crate::tile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub w: u8, // block
    pub x: u8, // col
    pub y: u8, // row
    pub z: u8, // index -- might have to disable
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub val: u8,
    pub access: Access,
    pub coord: Coord,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnitType {
    Block,
    Row,
    Col,
}

#[derive(Clone)]
pub struct Unit {
    pub data: Vec<u8>,
    pub full: u16,
    pub count: u8,
    //pub kind: UnitType
}

impl Coord {
    pub fn zeroed() -> Coord {
        return Coord {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        };
    }
}

impl std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unit {{ data: {:?}, full: {:#b}, count: {} }}",
            self.data, self.full, self.count
        )
    }
}

impl Unit {
    pub fn update(&mut self, tiles: &mut Vec<tile::Tile>) {
        self.full = 0;
        self.count = 0;
        for i in 0usize..9usize {
            let val: u8 = tiles[self.data[i] as usize].val;
            if val > 0 {
                self.full |= 1 << (val - 1);
                self.count += 1;
            }
        }
        //println!("updated presence: {:09b}->{:09b}", (!old) & 0x1FF, (!self.full) & 0x1FF);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Access {
    Default,
    LPNPass, // Last Possible Number
    LRCPass, // Last Remaining Cell
    CouplePass,
    CanEdit,
}
