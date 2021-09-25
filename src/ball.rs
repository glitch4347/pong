use macroquad::prelude::*;

use crate::types::*;
use crate::game::Game;
use crate::bar::Bar;


pub struct Ball {
    pub position: Point,
    pub dir: Point
}

impl Ball {
    pub fn render(&self, game: &Game) {
        game.render_pixel(self.position, WHITE);
    }

    pub fn tick(&mut self, bar_left: &Bar, bar_right: &Bar) {
        if self.position.1 as f32 + self.dir.1 >= GAME_HEIGHT as f32 {
            self.position.1 = GAME_HEIGHT as f32;
            self.dir.1 *= -1.;
        }
        
        if self.position.1 as f32 + self.dir.1 <= 0. {
            self.dir.1 *= -1.;
        }

        if bar_left.intersect(&self, true) {
            self.dir.0 *= -1.;
            // TODO: add momentum from bar movement
        }

        if bar_right.intersect(&self, false) {
            self.dir.0 *= -1.;
            // TODO: add momentum from bar movement
        }

        self.position.0 += self.dir.0;
        self.position.1 += self.dir.1;

    }
}