use crate::constants::RawColor;

pub struct Block {
    pub x: i32,
    pub y: i32,
    pub color: RawColor,
}

impl Block {
    pub fn new(x: i32, y: i32, color: RawColor) -> Block {
        Block { x, y, color }
    }
}
