use piston_window::*;

use crate::block::Block;
use crate::constants::BLOCK_SIZE;

pub fn transform_coord(coord: i32) -> f64 {
    (coord * BLOCK_SIZE) as f64
}

pub fn transform_coord_32(coord: i32) -> u32 {
    transform_coord(coord) as u32
}

pub fn draw_block(ctx: &Context, g2d: &mut G2d, block: &Block) {
    rectangle(
        block.color,
        [
            transform_coord(block.x),
            transform_coord(block.y),
            BLOCK_SIZE as f64,
            BLOCK_SIZE as f64,
        ],
        ctx.transform,
        g2d,
    );
}
