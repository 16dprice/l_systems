mod configurations;
mod l_system_drawing;
mod ui;

use std::f32::consts::PI;
use std::time::SystemTime;

use configurations::{l_system_configurations::{
    get_preset_l_system_configuration,
    PresetLSystemConfiguration
}, runtime_configuration::RuntimeConfiguration};
use l_system_drawing::l_system_drawing::get_l_system_lines;

use macroquad::prelude::*;
use ui::{colors::get_rainbow_color, input::handle_input, telemetry::{draw_custom_telemetry_data, CustomTelemetryData}};

fn main_conf() -> Conf {
    Conf {
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut runtime_configuration = RuntimeConfiguration {
        theta: PI / 2.0,
        delta_theta: 0.5,
        iterations: 1,

        color_percentage_offset: 0.0,

        camera_target: vec2(0.0, 0.0),
        camera_zoom: vec2(0.001, 0.001),
        camera_rotation: 0.0
    };

    loop {
        clear_background(BLACK);

        handle_input(&mut runtime_configuration);

        set_camera(&Camera2D {
            target: runtime_configuration.camera_target,
            zoom: runtime_configuration.camera_zoom,
            rotation: runtime_configuration.camera_rotation,
            ..Default::default()
        });

        let time_start = SystemTime::now();
        let lines = get_l_system_lines(
            get_preset_l_system_configuration(PresetLSystemConfiguration::My3),
            runtime_configuration.theta,
            runtime_configuration.iterations
        );
        let get_lines_time = SystemTime::now().duration_since(time_start).unwrap().as_micros();

        // it should get to one every 5 * fps frames
        // so if there's 300 frames, it should increase at 1/300 every frame
        // so delta = 1 / (5 * fps)
        runtime_configuration.color_percentage_offset += 1.0 / (5.0 * get_fps() as f32);
        if runtime_configuration.color_percentage_offset >= 1.0 { runtime_configuration.color_percentage_offset = 0.0; }

        let time_start = SystemTime::now();
        for idx in 0..lines.len() {
            let line = &lines[idx];

            let color_percentage = idx as f32 / lines.len() as f32;

            let mut final_color_percentage = color_percentage + runtime_configuration.color_percentage_offset;
            while final_color_percentage > 1.0 { final_color_percentage -= 1.0; }

            let start = BLUE;
            let end = PINK;

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
            theta: runtime_configuration.theta,
            delta_theta: runtime_configuration.delta_theta,
            iterations: runtime_configuration.iterations,
            get_lines_time,
            for_loop_time,
            color_percentage_offset: runtime_configuration.color_percentage_offset,
            num_lines: lines.len(),
        });

        next_frame().await;
    }
}