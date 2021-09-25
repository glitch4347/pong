use macroquad::prelude::*;

pub fn render_scores(a: u32, b: u32) {
    let font_size = 30.;
    let s = format!("{} | {}", a, b);
    let text1 = s.as_str();

    let text_size = measure_text(text1, None, font_size as _, 1.0);
    
    draw_text(
        text1,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height,
        font_size,
        DARKGRAY,
    );
}