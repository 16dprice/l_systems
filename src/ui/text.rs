use macroquad::{
    color::Color,
    text::{draw_text_ex, TextParams}
};

pub fn draw_percentage_text_ex(
    text: &str, x: f32, y: f32, percentage: f32, params: TextParams
) {
    let num_text_chars = text.chars().count() as f32;

    let mut next_x_offset = x;
    for (i, c) in text.chars().enumerate() {
        let idx = i as f32;
        if idx / num_text_chars < percentage {
            let mut a = 1.0;
            if (idx + 1.0) / num_text_chars > percentage {
                // (idx + a) / num_text_chars = text_percentage
                // idx + a = text_percentage * num_text_chars
                // a = text_percentage * num_text_chars - idx
                a = percentage * num_text_chars - idx;
            }

            let text_dimensions = draw_text_ex(
                format!("{}", c).as_str(), next_x_offset, y, TextParams {
                    color: Color {
                        r: params.color.r,
                        g: params.color.g,
                        b: params.color.b,
                        a
                    },
                    ..params
                }
            );
            next_x_offset = next_x_offset + text_dimensions.width;
        }
    }
}