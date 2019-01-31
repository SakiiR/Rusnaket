use crate::block::Block;
use crate::border::Border;
use crate::graphics::draw_block;
use crate::snake::{Direction, Snake};

use piston_window::*;
use rand::prelude::*;

use crate::constants::{Color, GAME_SPEED, HORIZONTAL_BLOCKS_COUNT, VERTICAL_BLOCKS_COUNT};

pub struct Game {
    snake: Snake,
    pub border: Border,
    time_buff: f64,
    food: Vec<Block>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            border: Border::new(),
            time_buff: 0.0,
            food: Vec::new(),
        }
    }

    fn add_food(&mut self, count: u32) {
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let x: i32 = rng.gen_range(1, HORIZONTAL_BLOCKS_COUNT - 1);
            let y: i32 = rng.gen_range(1, VERTICAL_BLOCKS_COUNT - 1);

            let block: Block = Block::new(x, y, Color::value(Color::Food));

            self.food.push(block);
        }
    }

    fn draw_food(&self, ctx: &Context, g2d: &mut G2d) {
        for block in self.food.iter() {
            draw_block(&ctx, g2d, block);
        }
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        self.snake.draw(ctx, g2d);
        self.border.draw(ctx, g2d);
        self.draw_food(ctx, g2d);
    }

    pub fn key(&mut self, key: Key) {
        let snake_direction = self.snake.get_direction();
        let key_direction: Direction = match key {
            Key::Up | Key::Z | Key::W => Direction::Up,
            Key::Down | Key::S => Direction::Down,
            Key::Right | Key::D => Direction::Right,
            Key::Left | Key::Q | Key::A => Direction::Left,
            _ => snake_direction,
        };

        // Check that the gamer is not trying to get in the opposite direction
        // This is not allowed by the snake RFC :p
        if self.snake.get_direction().opposite() == key_direction {
            return;
        }

        self.snake.set_direction(key_direction);
    }

    fn is_it_time_to_refresh(&self) -> bool {
        self.time_buff > GAME_SPEED
    }

    pub fn tick(&mut self, delta_time: f64) {
        self.time_buff += delta_time;

        self.border.randomize();
        if self.is_it_time_to_refresh() {
            self.snake.go();
            if self.food.len() == 0 {
                let mut rng = rand::thread_rng();

                self.add_food(rng.gen_range(1, 5));
            }

            self.time_buff = 0.0;
        }
    }
}
