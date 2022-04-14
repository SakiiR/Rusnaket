extern crate find_folder;

use crate::block::Block;
use crate::border::Border;
use crate::graphics::draw_block;
use crate::snake::{Direction, Snake};

use find_folder::Search;
use gfx_device_gl::Factory;
use piston_window::*;
use rand::prelude::*;
use texture::TextureSettings;

use crate::constants::{
    Color, HORIZONTAL_BLOCKS_COUNT, HORIZONTAL_LIMIT, INITIAL_GAME_SPEED, VERTICAL_BLOCKS_COUNT,
    VERTICAL_LIMIT,
};

pub struct Game {
    snake: Snake,
    pub border: Border,
    time_buff: f64,
    food: Vec<Block>,
    game_over: bool,
    factory: Factory,
    score: u32,
    game_speed: f64,
}

impl Game {
    pub fn new(factory: Factory) -> Game {
        Game {
            snake: Snake::new(),
            border: Border::new(),
            time_buff: 0.0,
            food: Vec::new(),
            game_over: false,
            factory: factory,
            score: 0,
            game_speed: INITIAL_GAME_SPEED,
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new();
        self.time_buff = 0.0;
        self.score = 0;
        self.game_over = false;
        self.game_speed = INITIAL_GAME_SPEED;
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

    fn draw_game_over(&self, ctx: &Context, g2d: &mut G2d) {
        // let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

        // let ref font = assets.join("monofur.ttf");

        // let mut glyphs = Glyphs::new(font, self.factory.clone(), TextureSettings::new()).unwrap();
        // let transform = ctx.transform.trans(2.0, 50.0);

        // text::Text::new_color([1.0, 0.0, 0.0, 1.0], 15)
        //     .draw("Game Over!", &mut glyphs, &ctx.draw_state, transform, g2d)
        //     .unwrap();
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        if self.game_over {
            self.draw_game_over(ctx, g2d);
            return;
        }
        self.snake.draw(ctx, g2d);
        self.border.draw(ctx, g2d);
        self.draw_food(ctx, g2d);
    }

    pub fn key(&mut self, key: Key) {
        if self.game_over {
            println!("Let's restart the game !");
            self.restart();
            return;
        }

        let snake_direction = self.snake.get_direction();
        let key_direction: Direction = match key {
            Key::Up | Key::Z | Key::W => Direction::Up,
            Key::Down | Key::S => Direction::Down,
            Key::Right | Key::D => Direction::Right,
            Key::Left | Key::Q | Key::A => Direction::Left,
            _ => snake_direction,
        };

        if snake_direction == key_direction {
            self.snake.go();
        }

        // Check that the gamer is not trying to get in the opposite direction
        // This is not allowed by the snake RFC :p
        if self.snake.get_direction().opposite() == key_direction {
            return;
        }

        self.snake.set_direction(key_direction);
    }

    fn is_it_time_to_refresh(&self) -> bool {
        self.time_buff > self.game_speed
    }

    fn is_it_game_over(&self) -> bool {
        let head: Block = self.snake.get_head();

        match head.x {
            1 => return true,
            HORIZONTAL_LIMIT => return true,
            _ => false,
        };

        match head.y {
            1 => return true,
            VERTICAL_LIMIT => return true,
            _ => false,
        };

        self.snake.is_eating_itself()
    }

    pub fn tick(&mut self, delta_time: f64) {
        self.time_buff += delta_time;

        self.border.randomize();

        // Real tick loginc
        if self.is_it_time_to_refresh() {
            // Only process if the snake isn't dead
            if !self.game_over {
                // Is the nake hit the border or itself
                if self.is_it_game_over() {
                    println!("Fuck, snake is dead :(");
                    self.game_over = true;
                }

                // On step for the snake
                self.snake.go();

                // Check if the snake is eating anything
                if self.snake.is_eating_food(&mut self.food) {
                    self.score += 1;
                    self.game_speed -= match self.score {
                        0...10 => 0.01,
                        _ => 0.0,
                    };
                    println!("Score: {}", self.score);
                    println!("Game Speed: {}\n", self.game_speed);
                }

                // Is it the time to add food ?
                if self.food.len() == 0 {
                    let mut rng = rand::thread_rng();

                    self.add_food(rng.gen_range(1, 5));
                }

                self.time_buff = 0.0;
            }
        }
    }
}
