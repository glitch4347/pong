use macroquad::prelude::*;

use pong::game::Game;
use pong::types;

#[macroquad::main("Pong")]
async fn main() {

    let mut game = Game::new();
    let mut last_update = get_time();
    let mut game_over = false;
    loop {
        if !game_over {
            game.handle_keys();
            if get_time() - last_update > types::GAME_SPEED {
                last_update = get_time();
                game_over = game.tick();
            }
            game.render();
        } else {
            clear_background(WHITE);
            let text1 = "  Game Over. Press [enter] to play again.";
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
