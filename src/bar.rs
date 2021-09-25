use macroquad::prelude::*;

use crate::game::Game;
use crate::ball::Ball;
use crate::types::*;


pub struct Bar {
    pub length: u32,
    pub offset: u32,
    pub x: u32,
    pub dir: i32
}

impl Bar {
    pub fn render(&self, game: &Game) {
        let from = self.offset;
        let to = self.offset + self.length;
        for i in from..to {
            game.render_pixel((self.x as f32, i as f32), WHITE);
        }
    }
    
    pub fn intersect(&self, ball: &Ball, left: bool) -> bool {
        if left {
            // we add 1 to show width of the bar
            if ball.position.0 > self.x as f32 + 1. {
                return false;
            }
        } else {
            if ball.position.0 < self.x as f32 {
                return false;
            }
        }
            
        if ball.position.1 < self.offset as f32 {
            return false;
        }
        return ball.position.1 <= (self.offset + self.length) as f32;
    }

    pub fn tick(&mut self) {
        if self.dir != 0 {
            if self.dir == -1 && self.offset > 1 {
                self.offset -= 1;
            }

            if self.dir == 1 && self.offset + self.length < GAME_HEIGHT {
                self.offset += 1;
            }
            self.dir = 0;
        }
    }
}