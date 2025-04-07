use crate::{
    configurations::runtime_configuration::RuntimeConfiguration,
    ui::colors::get_rainbow_color
};

use macroquad::{
    math::Vec2,
    shapes::draw_line
};

pub fn render_to_screen(lines: &Vec<[Vec2; 2]>, current_fps: f32, runtime_configuration: &mut RuntimeConfiguration) {
    // it should get to one every 5 * fps frames
    // so if there's 300 frames, it should increase at 1/300 every frame
    // so delta = 1 / (5 * fps)
    runtime_configuration.color_percentage_offset += 1.0 / (5.0 * current_fps as f32);
    if runtime_configuration.color_percentage_offset >= 1.0 { runtime_configuration.color_percentage_offset = 0.0; }

    for idx in 0..lines.len() {
        let line = &lines[idx];

        let color_percentage = idx as f32 / lines.len() as f32;

        let mut final_color_percentage = color_percentage + runtime_configuration.color_percentage_offset;
        while final_color_percentage > 1.0 { final_color_percentage -= 1.0; }

        // let line_color = interpolate_colors(BLUE, PINK, final_color_percentage);
        let line_color = get_rainbow_color(final_color_percentage);

        draw_line(
            line[0].x, line[0].y,
            line[1].x, line[1].y,
            2.0, line_color,
        );
    }
}
