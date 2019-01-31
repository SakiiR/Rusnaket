// Window size
pub const WINDOW_WIDTH: i32 = 250;
pub const WINDOW_HEIGHT: i32 = 250;

// Block size in pixel
pub const BLOCK_SIZE: i32 = 5;

// Number of blocks per side
pub const HORIZONTAL_BLOCKS_COUNT: i32 = WINDOW_WIDTH / BLOCK_SIZE;
pub const VERTICAL_BLOCKS_COUNT: i32 = WINDOW_HEIGHT / BLOCK_SIZE;

pub enum Color {
    Snake,
    Border,
}

pub type RawColorComponent = f32;
pub type RawColor = [RawColorComponent; 4];

impl Color {
    pub fn value(color: Color) -> RawColor {
        match color {
            Color::Snake => [0.5, 0.5, 0.5, 0.5],
            Color::Border => [0.0, 0.0, 0.0, 0.0],
        }
    }
}
