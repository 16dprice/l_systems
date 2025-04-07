mod configurations;
mod l_system_drawing;
mod ui;
mod rendering;

use configurations::{
    l_system_configurations::{get_preset_l_system_configuration, PresetLSystemConfiguration},
    runtime_configuration::RuntimeConfiguration
};
use l_system_drawing::l_system_drawing::get_l_system_lines;
use rendering::render_to_screen::{self, render_to_screen};
use ui::{
    colors::{get_rainbow_color, interpolate_colors},
    container::draw_container,
    input::handle_input,
    telemetry::{draw_custom_telemetry_data, CustomTelemetryData}
};

use macroquad::prelude::*;

fn main_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut runtime_configuration = RuntimeConfiguration {
        theta: 0.0,
        delta_theta: 0.5,
        iterations: 1,

        color_percentage_offset: 0.0,

        camera_target: vec2(0.0, 0.0),
        camera_zoom: vec2(0.001, 0.001),
        camera_rotation: 0.0,

        show_telemetry: true,
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

        let lines = get_l_system_lines(
            get_preset_l_system_configuration(PresetLSystemConfiguration::Hilbert),
            runtime_configuration.theta,
            runtime_configuration.iterations
        );

        render_to_screen(&lines, get_fps() as f32, &mut runtime_configuration);

        set_default_camera();

        if runtime_configuration.show_telemetry {
            draw_container(650.0, 300.0, 0.0, 0.0, 10.0, 10);

            draw_custom_telemetry_data(CustomTelemetryData {
                theta: runtime_configuration.theta,
                delta_theta: runtime_configuration.delta_theta,
                iterations: runtime_configuration.iterations,
                color_percentage_offset: runtime_configuration.color_percentage_offset,
                num_lines: lines.len(),
                camera_target: runtime_configuration.camera_target,
                camera_zoom: runtime_configuration.camera_zoom,
                camera_rotation: runtime_configuration.camera_rotation,
            });
        }

        next_frame().await;
    }
}