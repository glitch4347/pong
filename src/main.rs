use macroquad::prelude::*;


const PIXEL_WIDTH: u32 = 8;
const PIXEL_HEIGHT: u32 = 8;

const GAME_SCREEN_WIDTH: u32 = 1024;
const GAME_SCREEN_HEIGHT: u32 = 1024;

const GAME_WIDTH: u32 = GAME_SCREEN_WIDTH / PIXEL_WIDTH;
const GAME_HEIGHT: u32 = GAME_SCREEN_HEIGHT / PIXEL_HEIGHT;

const BAR_LENGTH: u32 = 50;


// it's i32 not u32 because we want to work as vectors
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
    length: u32,
    offset: u32,
    x: u32,
    dir: i32
}

impl Bar {
    fn render(&self, game: &Game) {
        let from = self.offset;
        let to = self.offset + self.length;
        for i in from..to {
            game.render_pixel((self.x as i32, i as i32), WHITE);
        }
    }

    fn intersect(&self, ball: &Ball) -> bool {
        if ball.position.0 != self.x as i32 {
            return false;
        }
        if ball.position.1 < self.offset as i32 {
            return false;
        }
        return ball.position.1 <= (self.offset + self.length) as i32;
    }

    fn tick(&mut self) {
        if self.dir != 0 {
            if self.dir == -1 && self.offset > 1 {
                self.offset -= 1;
            }

            if self.dir == 1 && self.offset + self.length + 1 < GAME_HEIGHT {
                self.offset += 1;
            }
            self.dir = 0;
        }
    }
}

        
struct Game {
    bar_left: Bar,
    bar_right: Bar,
    ball: Ball
}

impl Game {
    fn new() -> Game {

        let bar_left = Bar {
            length: BAR_LENGTH,
            offset: (GAME_HEIGHT - BAR_LENGTH) / 2,
            x: 0,
            dir: 0
        };

        let bar_right = Bar { 
            length: BAR_LENGTH,
            offset: (GAME_HEIGHT - BAR_LENGTH) / 2,
            x: GAME_WIDTH - 1,
            dir: 0
        };

        let ball = Ball {
            position: (GAME_WIDTH as i32 / 2, GAME_HEIGHT as i32 / 2),
            dir: (1, 1)
        };
        

        return Game {
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
        self.bar_left.tick();
        self.bar_right.tick();
        
        if self.ball.position.1 as i32 + self.ball.dir.1 >= GAME_HEIGHT as i32 {
            self.ball.position.1 = GAME_HEIGHT as i32;
            self.ball.dir.1 *= -1;
        }
        
        if self.ball.position.1 as i32 + self.ball.dir.1 <= 0 {
            self.ball.dir.1 *= -1;
        }

        if self.bar_left.intersect(&self.ball) {
            self.ball.dir.0 *= -1;
        }

        if self.bar_right.intersect(&self.ball) {
            self.ball.dir.0 *= -1;
        }

        self.ball.position.0 += self.ball.dir.0;
        self.ball.position.1 += self.ball.dir.1;

        if self.ball.position.0 < 0 || self.ball.position.0 > GAME_WIDTH as i32 {
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
                self.render_pixel((middle as i32, i as i32), GRAY);
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
