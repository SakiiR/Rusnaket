use crate::block::Block;
use crate::constants::{Color, HORIZONTAL_BLOCKS_COUNT, SNAKE_INITIAL_SIZE, VERTICAL_BLOCKS_COUNT};
use crate::graphics::draw_block;

use piston_window::*;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

pub struct Snake {
    body: Vec<Block>,
    head: Block,
    direction: Direction,
}

impl Snake {
    pub fn get_head(&self) -> Block {
        self.head
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn grow(&mut self) {
        self.body.push(Block::new(0, 0, Color::value(Color::Snake)));
    }

    pub fn is_eating_food(&mut self, food: &mut Vec<Block>) -> bool {
        let head: Block = self.get_head();

        for (i, block) in food.iter().enumerate() {
            if block == &head {
                food.remove(i);
                self.grow();
                return true;
            }
        }

        false
    }

    pub fn is_eating_itself(&self) -> bool {
        let head: Block = self.get_head();

        for block in self.body.iter() {
            if block == &head {
                return true;
            }
        }

        false
    }

    pub fn new() -> Snake {
        let mut body: Vec<Block> = Vec::new();

        // Create the first snake blocks
        for i in 0..SNAKE_INITIAL_SIZE {
            body.push(Block::new(
                // Center the snake
                // We also want the head block to be the first block in the Vec
                (HORIZONTAL_BLOCKS_COUNT / 2) + (SNAKE_INITIAL_SIZE / 2) - i,
                VERTICAL_BLOCKS_COUNT / 2,
                Color::value(Color::Snake),
            ));
        }

        Snake {
            body: body,
            head: Block::new(
                (HORIZONTAL_BLOCKS_COUNT / 2) + (SNAKE_INITIAL_SIZE / 2) + 1,
                VERTICAL_BLOCKS_COUNT / 2,
                Color::value(Color::SnakeHead),
            ),
            direction: Direction::Right,
        }
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        draw_block(&ctx, g2d, &self.head);
        for block in self.body.iter() {
            draw_block(&ctx, g2d, block);
        }
    }

    pub fn go(&mut self) {
        let mut head: &mut Block = &mut self.head;
        let mut last: Block = *head; // Make a copy

        // Move the head forward
        match self.direction {
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Left => head.x -= 1,
        }

        // Make the other blocks follow
        for mut block in self.body.iter_mut() {
            let tmp = *block;
            block.x = last.x;
            block.y = last.y;
            last = tmp;
        }
    }
}
