use macroquad::prelude::*;


type Point = (u16, u16);

struct Bar {
    length: u16,
    offset: u16
}

struct Game {
    pixel_width: u16,
    pixel_heigh: u16,
    game_screen_width: u16,
    game_screen_height: u16,
    game_width: u16,
    game_height: u16,
    player_left: Bar,
    player_right: Bar
}

impl Game {
    fn new() -> Game {

        let player_left = Bar {
            length: 10,
            offset: 5
        };

        let player_right = Bar {
            length: 10,
            offset: 5
        };

        return Game {
            pixel_width: 14,
            pixel_heigh: 14,
            game_screen_width: 658,
            game_screen_height: 406,
            game_width: 658 / 14,
            game_height: 406 / 14,
            player_left,
            player_right
        }
    }

    fn render(&self) {
        clear_background(BLACK);
        self.render_middle_border();
        self.render_bars();
    }

    fn render_pixel(&self, p: Point, color: Color) {
        let pw = screen_width() * (self. pixel_width as f32) / (self.game_screen_width as f32);
        let ph = screen_height() * (self. pixel_heigh as f32) / (self.game_screen_height as f32);
        let x = pw * (p.0 as f32);
        let y = ph * (p.1 as f32);
        draw_rectangle(x, y, pw, ph, color);
    }

    fn render_middle_border(&self) {
        let middle = self.game_width / 2;
        for i in 0..self.game_height {
            if i % 2 == 0 {
                self.render_pixel((middle, i), GRAY);
            }
        }
    }

    fn render_bars(&self) {
        for i in self.player_left.offset..self.player_left.length {
            self.render_pixel((0, i), WHITE);
        }
        for i in self.player_right.offset..self.player_left.length {
            self.render_pixel((self.game_width - 1, i), WHITE);
        }
    }
}

#[macroquad::main("Pong")]
async fn main() {

    let g= Game::new();

    loop {
        g.render();
        // TODO: fix window size
        next_frame().await
    }
}
