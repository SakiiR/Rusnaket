extern crate piston_window;

mod block;
mod border;
mod constants;
mod game;
mod graphics;
mod snake;

use game::Game;

use crate::constants::{
    BACKGROUND_COLOR, BLOCK_SIZE, HORIZONTAL_BLOCKS_COUNT, VERTICAL_BLOCKS_COUNT, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

use crate::graphics::transform_coord_32;

use piston_window::*;

fn debug() {
    println!("window_width: {}", WINDOW_WIDTH);
    println!("window_height: {}", WINDOW_HEIGHT);
    println!("block_size: {}", BLOCK_SIZE);
    println!("horizontal_blocks_count: {}", HORIZONTAL_BLOCKS_COUNT);
    println!("vertical_blocks_count: {}", VERTICAL_BLOCKS_COUNT);
}

fn main() {
    debug();

    let mut window: PistonWindow = WindowSettings::new(
        "Rusnaket!",
        [
            transform_coord_32(WINDOW_WIDTH),
            transform_coord_32(WINDOW_HEIGHT),
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game: Game = Game::new(window.factory.clone());

    // Event loop
    while let Some(event) = window.next() {
        // When piston is ready to draw
        window.draw_2d(&event, |context, graphics, _| {
            clear(BACKGROUND_COLOR, graphics);
            game.draw(&context, graphics);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key(key);
        }

        // When piston let us perform some "CPU" actions
        event.update(|arg| {
            game.tick(arg.dt);
        });
    }
}
