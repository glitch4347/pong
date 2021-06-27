use macroquad::prelude::*;


type Point = (i32, i32);

struct Ball {
    position: Point,
    dir: Point
}

impl Ball {
    fn render(&self, game: &Game) {
        game.render_pixel(self.position, WHITE);
    }
}


struct Bar {
    length: i32,
    offset: i32,
    x: i32,
    dir: i32
}

impl Bar {
    fn render(&self, game: &Game) {
        let from = self.offset;
        let to = self.offset + self.length;
        for i in from..to {
            game.render_pixel((self.x, i), WHITE);
        }
    }
    fn tick(&mut self, vertical_offset: i32) {
        if self.dir != 0 {
            if self.dir == -1 && self.offset > 1 {
                self.offset -= 1;
            }

            if self.dir == 1 && self.offset + self.length + 1 < vertical_offset {
                self.offset += 1;
            }
            self.dir = 0;
        }
    }
}

        
struct Game {
    pixel_width: i32,
    pixel_heigh: i32,
    game_screen_width: i32,
    game_screen_height: i32,
    game_width: i32,
    game_height: i32,
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
            x: 0,
            dir: 0
        };

        let bar_right = Bar { 
            length: bar_length, 
            offset: bar_offset,
            x: game_width - 1,
            dir: 0
        };

        let ball = Ball {
            position: (game_width / 2, game_height / 2),
            dir: (1, 1)
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

    fn handle_keys(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.bar_right.dir = -1;
        }
        if is_key_down(KeyCode::Down) {
            self.bar_right.dir = 1;
        }
        if is_key_down(KeyCode::Q) {
            self.bar_left.dir = -1;
        }
        if is_key_down(KeyCode::A) {
            self.bar_left.dir = 1;
        }
    }

    fn tick(&mut self) -> bool {
        self.bar_left.tick(self.game_height);
        self.bar_right.tick(self.game_height);
        
        if self.ball.position.1 as f32 + self.ball.dir.1 as f32 >= self.game_height as f32 {
            self.ball.dir.1 *= -1;
        }
        
        if self.ball.position.1 + self.ball.dir.1 <= 0 {
            self.ball.dir.1 *= -1;
        }

        if self.ball.position.0 == 1 && 
            self.ball.position.1 >= self.bar_left.offset &&
            self.ball.position.1 <= self.bar_left.offset + self.bar_left.length {
                self.ball.dir.0 *= -1;
            }

        if self.ball.position.0 + 1 == self.game_width && 
            self.ball.position.1 >= self.bar_right.offset &&
            self.ball.position.1 <= self.bar_right.offset + self.bar_right.length {
                self.ball.dir.0 *= -1;
            }

        self.ball.position.0 += self.ball.dir.0;
        self.ball.position.1 += self.ball.dir.1;

        if self.ball.position.0 < 0 || self.ball.position.0 > self.game_width {
            return true;
        } else {
            return false;
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

    let mut game= Game::new();
    let mut last_update = get_time();
    let speed = 0.1;
    let mut game_over = false;
    loop {
        if !game_over {
            game.handle_keys();
            if get_time() - last_update > speed {
                last_update = get_time();
                game_over = game.tick();
            }
            game.render();
        } else {
            clear_background(WHITE);
            let text1 = "Game Over. Press [enter] to play again.";
            let text2 = "[Q] and [A] to move left bar and arrows to move right bar";
            let font_size = 30.;
            let text_size = measure_text(text1, None, font_size as _, 1.0);

            draw_text(
                text1,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height,
                font_size,
                DARKGRAY,
            );

            draw_text(
                text2,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height,
                font_size * 0.75,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                game = Game::new();
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await
    }
}
