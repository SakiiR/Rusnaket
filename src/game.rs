use crate::border::Border;
use crate::snake::Snake;

use piston_window::*;

pub struct Game {
    snake: Snake,
    pub border: Border,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            border: Border::new(),
        }
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        self.snake.draw(ctx, g2d);
        self.border.draw(ctx, g2d);
    }

    pub fn tick(&mut self, _delta_time: f64) {
        self.border.randomize();
    }
}
