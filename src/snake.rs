use crate::block::Block;
use crate::constants::Color;
use crate::graphics::draw_block;

use piston_window::*;

pub struct Snake {
    blocks: Vec<Block>,
}

impl Snake {
    pub fn new() -> Snake {
        let mut blocks: Vec<Block> = Vec::new();

        // Create the first snake blocks
        blocks.push(Block::new(0, 0, Color::value(Color::Snake)));
        blocks.push(Block::new(0, 0, Color::value(Color::Snake)));
        blocks.push(Block::new(0, 0, Color::value(Color::Snake)));

        Snake { blocks: blocks }
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        for block in self.blocks.iter() {
            draw_block(&ctx, g2d, block);
        }
    }
}
