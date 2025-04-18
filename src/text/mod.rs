use macroquad::{
    color::Color,
    text::{
        draw_text_ex,
        measure_text,
        Font,
        TextParams
    }
};

pub struct DrawableText<'a> {
    pub lines: Vec<&'a str>,
    pub x: f32,
    pub y: f32,
    
    pub max_width: f32,
    pub max_font_size: u16,
    pub vertical_gap: f32,
    pub percentage_to_draw: f32,

    pub color: Color,
    pub font: Option<&'a Font>,
    pub font_scale: f32,
}

impl<'a> DrawableText<'a> {
    pub fn draw_box_text_ex(&self) {
        let mut max_possible_font_sizes: Vec<u16> = Vec::new();
    
        for (i, l) in self.lines.iter().enumerate() {
            max_possible_font_sizes.push(0);
            for font_size in 1..(self.max_font_size + 1) {
                let dimensions = measure_text(
                    *l,
                    self.font,
                    font_size,
                    self.font_scale
                );
                if dimensions.width < self.max_width {
                    max_possible_font_sizes[i] = font_size;
                }
            }
        }
    
        let mut min_font_size = self.max_font_size;
        for fs in &max_possible_font_sizes {
            if *fs < min_font_size { min_font_size = *fs; }
        }
    
        let mut total_chars_in_all_lines = 0;
        for l in &self.lines { total_chars_in_all_lines += (*l).chars().count(); }
    
        let mut y_offset = 0.0;
        let mut percentage_drawing_bounds = vec![(0.0, self.lines[0].chars().count() as f32 / total_chars_in_all_lines as f32)];
        for (i, l) in self.lines.iter().enumerate() {
            if i > 0 {
                percentage_drawing_bounds.push(
                    (
                        percentage_drawing_bounds[i - 1].1,
                        percentage_drawing_bounds[i - 1].1 + ((*l).chars().count() as f32 / total_chars_in_all_lines as f32)
                    )
                )
            }
    
            if self.percentage_to_draw < percentage_drawing_bounds[i].0 { continue; }
    
            self.draw_percentage_text_ex(
                *l,
                self.x, self.y + y_offset,
                (
                    (self.percentage_to_draw - percentage_drawing_bounds[i].0) /
                    (percentage_drawing_bounds[i].1 - percentage_drawing_bounds[i].0)
                )

            );
            y_offset += measure_text(*l, self.font, min_font_size, self.font_scale).height + self.vertical_gap;
        }
    }

    fn draw_percentage_text_ex(
        &self, text: &str, x: f32, y: f32, percentage: f32,
    ) -> f32 {
        let num_text_chars = text.chars().count() as f32;
        let lag = 5.0;
        let mut next_x_offset = x;
    
        for i in 0..(text.chars().count() + lag as usize) {
            let idx = i as f32;
            if idx / num_text_chars < percentage {
                let mut a = 1.0;
                if (idx + lag) / (num_text_chars + lag) > percentage {
                    // (idx + a * lag) / (num_text_chars + lag) = percentage
                    // idx + a * lag = percentage * (num_text_chars + lag)
                    // a = (percentage * (num_text_chars + lag) - idx) / lag
                    a = (percentage * (num_text_chars + lag) - idx) / lag;
                }
    
                let c = text.chars().nth(i);
                if let Some(c) = c {
                    let text_dimensions = draw_text_ex(
                        format!("{}", c).as_str(), next_x_offset, y, TextParams {
                            color: Color {
                                r: self.color.r,
                                g: self.color.g,
                                b: self.color.b,
                                a
                            },
                            font: self.font,
                            font_scale: self.font_scale,
                            ..Default::default()
                        }
                    );
                    next_x_offset = next_x_offset + text_dimensions.width;
                }
            }
        }
    
        return next_x_offset;
    }

}
