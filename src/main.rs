#![allow(unused)]
mod configurations;
mod ui;
mod rendering;
mod animation;
mod examples;
mod videos;
mod l_system;

use core::num;
use std::f32::consts::PI;

use configurations::{
    l_system_configurations::{get_preset_l_system_configuration, LSystemConfiguration, PresetLSystemConfiguration},
    runtime_configuration::RuntimeConfiguration
};
use indicatif::ProgressBar;
use l_system::{LSystem, ScaleParams};
use rendering::{render_to_png::{create_render_target_and_set_camera, get_width_and_height_from_resolution, render_lines_to_png, Resolution}, render_to_screen::{render_lines_to_screen, render_telemetry_to_screen}};
use ui::{
    colors::{get_rainbow_color, interpolate_colors},
    container::draw_container,
    input::handle_input,
    telemetry::{draw_custom_telemetry_data, CustomTelemetryData}, text::{draw_box_text_ex, draw_percentage_text_ex, BoxTextParams}
};

use macroquad::prelude::*;

const BACKGROUND_COLOR: Color = color_u8!(0x18, 0x18, 0x18, 0xFF);
const RESOLUTION: (i32, i32) = (800, 600);

fn main_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        window_width: RESOLUTION.0,
        window_height: RESOLUTION.1,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let resolution = Resolution::Low;
    let (width, height) = get_width_and_height_from_resolution(&resolution);

    let camera_target = vec2(0.0, 0.0);
    let camera_zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
    let desired_fps = 30;
    let total_seconds_of_video = 30;

    let mut camera = Camera2D {
        target: camera_target,
        zoom: camera_zoom,
        // rotation: 180.0,
        ..Default::default()
    };

    let mut l_system = LSystem {
        iterations: 5,
        l_system_configuration: get_preset_l_system_configuration(PresetLSystemConfiguration::My3),
        theta: 0.1,
        start_position: vec2(0.0, 0.0),
        scale_params: ScaleParams {
            width: width as f32,
            height: height as f32,
        }
    };

    // let progress_bar = ProgressBar::new(desired_fps * total_seconds_of_video);

    // for frame in 0..(desired_fps * total_seconds_of_video) {
    loop {
        // theta from 0 to PI / 5.0
        // theta = (PI / 5.0) * ((frame + 1) as f32) / (desired_fps as f32 * total_seconds_of_video as f32);
        
        // let render_target = create_render_target_and_set_camera(&mut camera, &resolution);
        clear_background(BACKGROUND_COLOR);

        camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
        set_camera(&camera);

        let lines = l_system.get_lines();
        for idx in 0..lines.len() {
            let line = &lines[idx];
            let color_percentage = idx as f32 / lines.len() as f32;
            let line_color = get_rainbow_color(color_percentage);
    
            draw_line(
                line[0].x, line[0].y,
                line[1].x, line[1].y,
                2.0, line_color,
            );
        }

        set_default_camera();
        
        // let image = render_target.texture.get_texture_data();
        // image.export_png(format!("./output/video_0/animation_1/frames/frame_{:>05}.png", frame).as_str());

        // progress_bar.inc(1);
        next_frame().await;
    }
}
