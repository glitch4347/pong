use macroquad::prelude::*;

use crate::game::Game;
use crate::ball::Ball;
use crate::types::*;


pub struct Bar {
    pub length: u32,
    pub offset: f32,
    pub x: u32,
    pub dir: f32
}

impl Bar {
    pub fn render(&self, game: &Game) {
        for i in 0..self.length {
            game.render_pixel((self.x as f32, self.offset + i as f32), WHITE);
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
        return ball.position.1 <= (self.offset + self.length as f32);
    }

    pub fn tick(&mut self) {

        self.offset += self.dir;

        if self.offset + self.length as f32 >= GAME_HEIGHT as f32 {
            self.offset = (GAME_HEIGHT - self.length) as f32;
        }
        
        if self.offset < 0. {
            self.offset = 0.;
        }

        self.dir = 0.;
        
    }
}