use macroquad::{
    color::{Color, WHITE},
    text::{draw_text, draw_text_ex, measure_text, Font, TextDimensions, TextParams}
};

pub fn draw_percentage_text_ex(
    text: &str, x: f32, y: f32, percentage: f32, params: TextParams
) -> f32 {
    let num_text_chars = text.chars().count() as f32;
    let lag = 5.0;
    let mut next_x_offset = x;

    for i in 0..(text.chars().count() + lag as usize) {
        let idx = i as f32;
        if idx / num_text_chars < percentage {
            let mut a = 1.0;
            if (idx + lag) / num_text_chars > percentage {
                // (idx + a * lag) / num_text_chars = text_percentage
                // idx + a * lag = text_percentage * num_text_chars
                // a = (text_percentage * num_text_chars - idx) / lag
                a = (percentage * num_text_chars - idx) / lag;
            }

            let c = text.chars().nth(i);
            if let Some(c) = c {
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

    return next_x_offset;
}

#[derive(Debug, Clone)]
pub struct BoxTextParams<'a> {
    pub max_width: f32,
    pub max_font_size: u16,
    pub vertical_gap: f32,
    pub text_params: TextParams<'a>,
}

pub fn draw_box_text_ex(
    lines: Vec<&str>, x: f32, y: f32, params: BoxTextParams
) {
    let mut max_possible_font_sizes: Vec<u16> = Vec::new();

    for (i, l) in lines.iter().enumerate() {
        max_possible_font_sizes.push(0);
        for font_size in 1..(params.max_font_size + 1) {
            let dimensions = measure_text(
                *l,
                params.text_params.font,
                font_size,
                params.text_params.font_scale
            );
            if dimensions.width < params.max_width {
                max_possible_font_sizes[i] = font_size;
            }
        }
    }

    let mut min_font_size = params.max_font_size;
    for fs in &max_possible_font_sizes {
        if *fs < min_font_size { min_font_size = *fs; }
    }

    let mut y_offset = 0.0;
    for l in lines {
        let dimensions = draw_text_ex(
            l,
            x, y + y_offset,
            TextParams {
                font_size: min_font_size,
                ..params.text_params
            }
        );
        y_offset += dimensions.height + params.vertical_gap;
    }
}
