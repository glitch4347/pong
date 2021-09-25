pub const PIXEL_WIDTH: u32 = 8;
pub const PIXEL_HEIGHT: u32 = 8;

pub const GAME_SCREEN_WIDTH: u32 = 1024;
pub const GAME_SCREEN_HEIGHT: u32 = 1024;

pub const GAME_WIDTH: u32 = GAME_SCREEN_WIDTH / PIXEL_WIDTH;
pub const GAME_HEIGHT: u32 = GAME_SCREEN_HEIGHT / PIXEL_HEIGHT;

pub const BAR_LENGTH: u32 = 50;

pub const GAME_SPEED: f64 = 0.008;

pub const BAR_SPEED: f32 = 0.3;

pub const MAX_SCORE: u32 = 10;

pub type Point = (f32, f32);

pub const  BALL_DIR: Point = (0.3, 0.1);

pub enum GameState {
    WinA,
    WinB,
    Continue
}