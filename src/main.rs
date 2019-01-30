extern crate piston_window;

mod game;

use game::Game;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game: Game = Game::new();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {});
    }
}
