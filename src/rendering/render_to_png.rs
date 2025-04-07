use macroquad::prelude::*;

use crate::{configurations::runtime_configuration::RuntimeConfiguration, ui::colors::get_rainbow_color};

pub fn render_lines_to_png(lines: &Vec<[Vec2; 2]>, current_fps: f32, runtime_configuration: &mut RuntimeConfiguration, png_path: &str) {
    let (width, height) = (3840, 2160);
    let render_target = render_target(width, height);
    render_target.texture.set_filter(FilterMode::Nearest);

    set_camera(&Camera2D {
        target: runtime_configuration.camera_target,
        zoom: runtime_configuration.camera_zoom,
        rotation: runtime_configuration.camera_rotation,
        render_target: Some(render_target.clone()),
        ..Default::default()
    });

    clear_background(BLACK);

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

    // it should get to one every 5 * fps frames
    // so if there's 300 frames, it should increase at 1/300 every frame
    // so delta = 1 / (5 * fps)
    runtime_configuration.color_percentage_offset += 1.0 / (5.0 * current_fps as f32);
    if runtime_configuration.color_percentage_offset >= 1.0 { runtime_configuration.color_percentage_offset = 0.0; }

    set_default_camera();

    let image = render_target.texture.get_texture_data();
    image.export_png(png_path);
}