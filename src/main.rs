use macroquad::prelude::*;

struct Game {
    pixel_width: u16,
    pixel_heigh: u16,
    game_width: u16,
    game_height: u16
}

type Point = (u16, u16);

impl Game {
    fn new() -> Game {
        return Game {
            pixel_width: 14,
            pixel_heigh: 14,
            game_width: 658,
            game_height: 400
        }
    }

    fn render(&self) {
        clear_background(BLACK);
        self.render_pixel((10,0));
    }

    fn render_pixel(&self, p: Point) {
        let pw = screen_width() * (self. pixel_width as f32) / (self.game_width as f32);
        let ph = screen_height() * (self. pixel_heigh as f32) / (self.game_height as f32);
        let x = pw * (p.0 as f32);
        let y = ph * (p.1 as f32);
        draw_rectangle(x, y, pw, ph, WHITE);
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
