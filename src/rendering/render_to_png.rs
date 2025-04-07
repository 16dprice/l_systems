use macroquad::prelude::*;

use crate::{configurations::runtime_configuration::RuntimeConfiguration, ui::colors::get_rainbow_color};

pub fn render_lines_to_png(lines: &Vec<[Vec2; 2]>, runtime_configuration: &RuntimeConfiguration, png_path: &str) {
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

    set_default_camera();

    let image = render_target.texture.get_texture_data();
    image.export_png(png_path);
}