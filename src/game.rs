use macroquad::prelude::*;

use crate::types::*;
use crate::bar::Bar;
use crate::ball::Ball;


pub struct Game {
    bar_left: Bar,
    bar_right: Bar,
    ball: Ball
}

impl Game {
    pub fn new() -> Game {

        let bar_left = Bar {
            length: BAR_LENGTH,
            offset: (GAME_HEIGHT - BAR_LENGTH) as f32 / 2.,
            x: 0,
            dir: 0.
        };

        let bar_right = Bar { 
            length: BAR_LENGTH,
            offset: (GAME_HEIGHT - BAR_LENGTH) as f32 / 2.,
            x: GAME_WIDTH - 1,
            dir: 0.
        };

        let ball = Ball {
            position: (GAME_WIDTH as f32 / 2., GAME_HEIGHT as f32 / 2.),
            dir: BALL_DIR
        };

        return Game {
            bar_left,
            bar_right,
            ball
        }
    }

    pub fn handle_keys(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.bar_right.dir = -BAR_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            self.bar_right.dir = BAR_SPEED;
        }
        if is_key_down(KeyCode::Q) {
            self.bar_left.dir = -BAR_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.bar_left.dir = BAR_SPEED;
        }
    }

    pub fn tick(&mut self) -> bool {
        self.bar_left.tick();
        self.bar_right.tick();
        self.ball.tick(&self.bar_left, &self.bar_right);

        if self.ball.position.0 < 0. || self.ball.position.0 > GAME_WIDTH as f32 {
            return true;
        } else {
            return false;
        }

    }

    pub fn render(&self) {
        clear_background(BLACK);
        self.render_borders();
        self.render_bars();
        self.ball.render(&self);
    }

    pub fn render_pixel(&self, p: Point, color: Color) {
        let pw = screen_width() * (PIXEL_WIDTH as f32) / (GAME_SCREEN_WIDTH as f32);
        let ph = screen_height() * (PIXEL_HEIGHT as f32) / (GAME_SCREEN_HEIGHT as f32);
        let x = pw * (p.0 as f32);
        let y = ph * (p.1 as f32);
        draw_rectangle(x, y, pw, ph, color);
    }

    fn render_borders(&self) {
        let middle = GAME_WIDTH / 2;
        for i in 0..GAME_HEIGHT {
            if i % 2 == 0 {
                self.render_pixel((middle as f32, i as f32), GRAY);
            }
        }
        draw_line(
            0., 0., screen_width() , 0., 
            PIXEL_HEIGHT as f32, GRAY
        );
        draw_line(
            0., screen_height(), screen_width() , screen_height(), 
            PIXEL_HEIGHT as f32, GRAY
        );
    }

    fn render_bars(&self) {
        self.bar_left.render(&self);
        self.bar_right.render(&self);
    }
}