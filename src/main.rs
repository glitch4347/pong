use macroquad::prelude::*;

use pong::game::Game;
use pong::scores::render_scores;
use pong::types;


#[macroquad::main("Pong")]
async fn main() {

    let mut game = Game::new();
    let mut last_update = get_time();
    let mut game_over = false;

    let mut score_a = 0u32;
    let mut score_b = 0u32;

    loop {
        if !game_over {
            game.handle_keys();
            if get_time() - last_update > types::GAME_SPEED {
                last_update = get_time();
                let game_state = game.tick();
                match game_state {
                    types::GameState::Continue => continue,
                    types::GameState::WinA => { 
                        score_a += 1;
                        game = Game::new();
                        last_update = get_time();
                    },
                    types::GameState::WinB => {
                        score_b += 1;
                        game = Game::new();
                        last_update = get_time();
                    },
                }
                if score_a == types::MAX_SCORE || score_b == types::MAX_SCORE {
                    game_over = true;
                    continue;
                }
            }
            game.render();
            render_scores(score_a, score_b);
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
                score_a = 0;
                score_b = 0;
            }
        }
        next_frame().await
    }
}
