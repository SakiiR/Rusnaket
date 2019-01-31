use piston_window::types::Color as PColor;

// Window size
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;

// Block size in pixel
pub const BLOCK_SIZE: i32 = 20;

// Number of blocks per side
pub const HORIZONTAL_BLOCKS_COUNT: i32 = WINDOW_WIDTH / BLOCK_SIZE;
pub const VERTICAL_BLOCKS_COUNT: i32 = WINDOW_HEIGHT / BLOCK_SIZE;

// The window background color
pub const BACKGROUND_COLOR: PColor = [0.108, 0.152, 0.527, 1.0];

// The Snake initial size
pub const SNAKE_INITIAL_SIZE: i32 = 10;

pub const GAME_SPEED: f64 = 0.1;

pub enum Color {
    Snake,
    SnakeHead,
    Food,
    Border,
}

pub type RawColorComponent = f32;
pub type RawColor = [RawColorComponent; 4];

impl Color {
    pub fn value(color: Color) -> RawColor {
        match color {
            Color::Snake => [0.0, 1.0, 0.0, 1.0],
            Color::SnakeHead => [0.0, 1.0, 1.0, 1.0],
            Color::Food => [0.8, 1.0, 0.0, 1.0],
            Color::Border => [0.8, 1.0, 0.0, 1.0],
        }
    }
}
