use piston_window::*;
use rand::prelude::*;

use crate::block::Block;
use crate::constants::{Color, HORIZONTAL_BLOCKS_COUNT, VERTICAL_BLOCKS_COUNT};
use crate::graphics::draw_block;

fn create_block(x: i32, y: i32) -> Block {
    Block::new(x, y, Color::value(Color::Border))
}

pub struct Border {
    blocks: Vec<Block>,
}

impl Border {
    pub fn new() -> Border {
        let mut blocks: Vec<Block> = Vec::new();

        // Create the left side blocks
        for y in 0..VERTICAL_BLOCKS_COUNT {
            blocks.push(create_block(0, y));
        }

        // Create the right side blocks
        for y in 0..VERTICAL_BLOCKS_COUNT {
            blocks.push(create_block(HORIZONTAL_BLOCKS_COUNT - 1, y));
        }

        // Create the bottom side blocks
        for x in 0..HORIZONTAL_BLOCKS_COUNT {
            blocks.push(create_block(x, VERTICAL_BLOCKS_COUNT - 1));
        }

        // Create the top side blocks
        for x in 0..HORIZONTAL_BLOCKS_COUNT {
            blocks.push(create_block(x, 0));
        }

        Border { blocks }
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        for block in self.blocks.iter() {
            draw_block(&ctx, g2d, block);
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for block in self.blocks.iter_mut() {
            (*block).color = [rng.gen(), rng.gen(), rng.gen(), 1.0];
        }
    }
}
