use macroquad::prelude::*;


type Point = (u16, u16);

struct Ball {
    position: Point
}

impl Ball {
    fn render(&self, game: &Game) {
        game.render_pixel(self.position, WHITE);
    }
}


struct Bar {
    length: u16,
    offset: u16,
    x: u16
}

impl Bar {
    fn render(&self, game: &Game) {
        let from = self.offset;
        let to = self.offset + self.length;
        for i in from..to {
            game.render_pixel((self.x, i), WHITE);
        }
    }
}

        
struct Game {
    pixel_width: u16,
    pixel_heigh: u16,
    game_screen_width: u16,
    game_screen_height: u16,
    game_width: u16,
    game_height: u16,
    bar_left: Bar,
    bar_right: Bar,
    ball: Ball
}

impl Game {
    fn new() -> Game {

        let pixel_width = 14;
        let pixel_heigh = 14;
        let game_screen_width = 658;
        let game_screen_height = 406;
        let game_width = 658 / 14;
        let game_height = 406 / 14;

        let bar_length = 10;
        let bar_offset = 8;

        let bar_left = Bar { 
            length: bar_length, 
            offset: bar_offset,
            x: 0
        };

        let bar_right = Bar { 
            length: bar_length, 
            offset: bar_offset,
            x: game_width - 1
        };

        let ball = Ball {
            position: (game_width / 2, game_height / 2)
        };
        

        return Game {
            pixel_width,
            pixel_heigh,
            game_screen_width,
            game_screen_height,
            game_width,
            game_height,
            bar_left,
            bar_right,
            ball
        }
    }

    fn render(&self) {
        clear_background(BLACK);
        self.render_borders();
        self.render_bars();
        self.ball.render(&self);
    }

    fn render_pixel(&self, p: Point, color: Color) {
        let pw = screen_width() * (self. pixel_width as f32) / (self.game_screen_width as f32);
        let ph = screen_height() * (self. pixel_heigh as f32) / (self.game_screen_height as f32);
        let x = pw * (p.0 as f32);
        let y = ph * (p.1 as f32);
        draw_rectangle(x, y, pw, ph, color);
    }

    fn render_borders(&self) {
        let middle = self.game_width / 2;
        for i in 0..self.game_height {
            if i % 2 == 0 {
                self.render_pixel((middle, i), GRAY);
            }
        }
        draw_line(
            0., 0., screen_width() , 0., 
            self.pixel_heigh as f32, GRAY
        );
        draw_line(
            0., screen_height(), screen_width() , screen_height(), 
            self.pixel_heigh as f32, GRAY
        );
    }

    fn render_bars(&self) {
        self.bar_left.render(&self);
        self.bar_right.render(&self);
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