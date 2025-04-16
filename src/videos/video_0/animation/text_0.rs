use macroquad::prelude::*;

use crate::ui::text::{draw_box_text_ex, BoxTextParams};

const BACKGROUND_COLOR: Color = color_u8!(0x18, 0x18, 0x18, 0xFF);

pub async fn text0() {
    let font = load_ttf_font("./fonts/euler.otf").await.unwrap();
    let mut text_percentage = 0.0;

    loop {
        clear_background(BACKGROUND_COLOR);

        text_percentage += get_frame_time() / 4.0;

        draw_box_text_ex(
            vec![
                "V = { F, +, -, [, ] }",
                "omega = F",
                "P = { F = F[-F-F][+F+F]F }"
            ], 
            screen_width() / 2.0 - 200.0,
            screen_height() / 2.0 - 100.0,
            BoxTextParams {
                max_font_size: 40,
                max_width: 800.0,
                vertical_gap: 10.0,
                percentage_to_draw: text_percentage,
                text_params: TextParams {
                    font: Some(&font),
                    color: WHITE,
                    ..Default::default()
                }
            }
        );

        next_frame().await;
    }
}
