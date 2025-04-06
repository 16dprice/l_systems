mod configurations;
mod l_system_drawing;
mod ui;

use std::f32::consts::PI;
use std::time::SystemTime;

use configurations::l_system_configurations::{
    get_preset_l_system_configuration,
    PresetLSystemConfiguration
};
use l_system_drawing::l_system_drawing::get_l_system_lines;

use macroquad::prelude::*;
use ui::telemetry::{draw_custom_telemetry_data, CustomTelemetryData};

fn main_conf() -> Conf {
    Conf {
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut theta = PI / 2.0;
    let mut delta_theta = 0.003;
    let mut iterations = 1;
    let mut color_percentage_offset = 0.0;

    let mut camera_target = vec2(0.0, 0.0);
    let mut camera_zoom = vec2(0.001, 0.001);
    let mut camera_rotation = 0.0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) { theta += delta_theta; }
        if is_key_down(KeyCode::Left) { theta -= delta_theta; }
        if is_key_down(KeyCode::Up) { delta_theta += 0.00001; }
        if is_key_down(KeyCode::Down) { delta_theta -= 0.00001; }

        if is_key_pressed(KeyCode::RightBracket) { iterations += 1; }
        if is_key_pressed(KeyCode::LeftBracket) { if iterations > 1 { iterations -= 1; } }

        if is_key_down(KeyCode::W) { camera_target.y -= 5.0; }
        if is_key_down(KeyCode::S) { camera_target.y += 5.0; }
        if is_key_down(KeyCode::A) {camera_target.x -= 5.0; }
        if is_key_down(KeyCode::D) { camera_target.x += 5.0; }
        if is_key_down(KeyCode::I) { camera_zoom.x += 0.0001; camera_zoom.y += 0.0001; }
        if is_key_down(KeyCode::O) { camera_zoom.x -= 0.0001; camera_zoom.y -= 0.0001; }
        if is_key_down(KeyCode::E) { camera_rotation -= 1.0; }
        if is_key_down(KeyCode::R) { camera_rotation += 1.0; }

        set_camera(&Camera2D {
            target: camera_target,
            zoom: camera_zoom,
            rotation: camera_rotation,
            ..Default::default()
        });

        let time_start = SystemTime::now();
        let lines = get_l_system_lines(
            get_preset_l_system_configuration(PresetLSystemConfiguration::Sierpinski),
            theta,
            iterations
        );
        let get_lines_time = SystemTime::now().duration_since(time_start).unwrap().as_micros();

        // it should get to one every 5 * fps frames
        // so if there's 300 frames, it should increase at 1/300 every frame
        // so delta = 1 / (5 * fps)
        color_percentage_offset += 1.0 / (5.0 * get_fps() as f32);
        if color_percentage_offset >= 1.0 { color_percentage_offset = 0.0; }

        let time_start = SystemTime::now();
        for idx in 0..lines.len() {
            let line = &lines[idx];

            let color_percentage = idx as f32 / lines.len() as f32;

            let mut final_color_percentage = color_percentage + color_percentage_offset;
            while final_color_percentage > 1.0 { final_color_percentage -= 1.0; }

            let start = RED;
            let end = BLACK;

            // let r = (1.0 - color_percentage) * start.r + color_percentage * end.r;
            // let g = (1.0 - color_percentage) * start.g + color_percentage * end.g;
            // let b = (1.0 - color_percentage) * start.b + color_percentage * end.b;

            let r = (1.0 - final_color_percentage) * start.r + final_color_percentage * end.r;
            let g = (1.0 - final_color_percentage) * start.g + final_color_percentage * end.g;
            let b = (1.0 - final_color_percentage) * start.b + final_color_percentage * end.b;

            draw_line(
                line[0].x, line[0].y,
                line[1].x, line[1].y,
                2.0,
                Color { r, g, b, a: 1.0 },
                // WHITE,
                // get_rainbow_color(final_color_percentage)
            );
        }
        let for_loop_time = SystemTime::now().duration_since(time_start).unwrap().as_micros();

        set_default_camera();
        draw_custom_telemetry_data(CustomTelemetryData {
            theta, delta_theta, iterations, get_lines_time, for_loop_time, color_percentage_offset
        });

        next_frame().await;
    }
}