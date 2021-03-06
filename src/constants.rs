use piston_window::types::Color as PColor;

// Window size
pub const WINDOW_WIDTH: i32 = 200;
pub const WINDOW_HEIGHT: i32 = 200;

// Block size in pixel
pub const BLOCK_SIZE: i32 = 5;

// Number of blocks per side
pub const HORIZONTAL_BLOCKS_COUNT: i32 = WINDOW_WIDTH / BLOCK_SIZE;
pub const VERTICAL_BLOCKS_COUNT: i32 = WINDOW_HEIGHT / BLOCK_SIZE;

pub const HORIZONTAL_LIMIT: i32 = HORIZONTAL_BLOCKS_COUNT - 1;
pub const VERTICAL_LIMIT: i32 = VERTICAL_BLOCKS_COUNT - 1;

// The window background color
pub const BACKGROUND_COLOR: PColor = [0.108, 0.152, 0.527, 1.0];

// The Snake initial size
pub const SNAKE_INITIAL_SIZE: i32 = 10;

// The initial game speed
pub const INITIAL_GAME_SPEED: f64 = 0.2;

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
