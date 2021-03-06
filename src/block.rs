use crate::constants::RawColor;

#[derive(Copy, Clone)]
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

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.x == other.x && self.y == other.y
    }
}
