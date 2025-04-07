mod configurations;
mod l_system_drawing;
mod ui;
mod rendering;

use std::f32::consts::PI;

use configurations::{
    l_system_configurations::{get_preset_l_system_configuration, PresetLSystemConfiguration},
    runtime_configuration::RuntimeConfiguration
};
use l_system_drawing::l_system_drawing::get_l_system_lines;
use rendering::{render_to_png::render_lines_to_png, render_to_screen::{render_lines_to_screen, render_telemetry_to_screen}};
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
    let theta_bounds = (0.0, PI / 2.0);

    let mut runtime_configuration = RuntimeConfiguration {
        theta: theta_bounds.0,
        delta_theta: 0.5,
        iterations: 6,

        color_percentage_offset: 0.0,

        camera_target: vec2(0.0, 0.0),
        camera_zoom: vec2(0.003, 0.003),
        camera_rotation: 0.0,

        show_telemetry: true,
    };

    for frame in 0..300 {
        let lines = get_l_system_lines(
            get_preset_l_system_configuration(PresetLSystemConfiguration::Hilbert),
            runtime_configuration.theta,
            runtime_configuration.iterations
        );

        render_lines_to_png(
            &lines,
            60.0,
            &mut runtime_configuration,
            format!("./data/first_movie/frame_{:0>5}.png", frame).as_str()
        );

        runtime_configuration.theta = (theta_bounds.1 - theta_bounds.0) * ((frame + 1) as f32 / 300.0) + theta_bounds.0;

        println!("Finished Frame: {}", frame);
    }
    
    // ffmpeg -framerate 60 -i frame_%05d.png -c:v libx264 -pix_fmt yuv420p ../output.mp4

    // loop {
    //     clear_background(BLACK);

    //     handle_input(&mut runtime_configuration);

    //     set_camera(&Camera2D {
    //         target: runtime_configuration.camera_target,
    //         zoom: runtime_configuration.camera_zoom,
    //         rotation: runtime_configuration.camera_rotation,
    //         ..Default::default()
    //     });

        // let lines = get_l_system_lines(
        //     get_preset_l_system_configuration(PresetLSystemConfiguration::Hilbert),
        //     runtime_configuration.theta,
        //     runtime_configuration.iterations
        // );

    //     render_lines_to_screen(&lines, get_fps() as f32, &mut runtime_configuration);

    //     set_default_camera();

    //     if runtime_configuration.show_telemetry {
    //         render_telemetry_to_screen(&runtime_configuration, lines.len());
    //     }

    //     next_frame().await;
    // }
}
