#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Coord {
    pub w: u8, // block
    pub x: u8, // col
    pub y: u8, // row
    pub z: u8, // index -- might have to disable
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Tile {
    pub val: u8,
    pub can_edit: bool,
    pub coord: Coord,
}

#[allow(dead_code)]
pub enum TileLoc {
    Row,
    Col,
    Block,
}
