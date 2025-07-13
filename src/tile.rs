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
#[derive(Eq, PartialEq)]
pub enum Unit {
    Block,
    Row,
    Col,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Access {
    Default,
    Step1,
    Step2,
    CanEdit
}
