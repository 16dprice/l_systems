#![allow(unused)]
mod configurations;
mod l_system_drawing;
mod ui;
mod rendering;
mod animation;
mod examples;

use core::num;
use std::f32::consts::PI;

use configurations::{
    l_system_configurations::{get_preset_l_system_configuration, LSystemConfiguration, PresetLSystemConfiguration},
    runtime_configuration::RuntimeConfiguration
};
use l_system_drawing::l_system_drawing::get_l_system_lines;
use rendering::{render_to_png::{create_render_target_and_set_camera, render_lines_to_png}, render_to_screen::{render_lines_to_screen, render_telemetry_to_screen}};
use ui::{
    colors::{get_rainbow_color, interpolate_colors},
    container::draw_container,
    input::handle_input,
    telemetry::{draw_custom_telemetry_data, CustomTelemetryData}, text::draw_percentage_text_ex
};

use macroquad::prelude::*;

const BACKGROUND_COLOR: Color = color_u8!(0x18, 0x18, 0x18, 0xFF);

fn main_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut camera = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(0.001, 0.001),
        rotation: 180.0,
        ..Default::default()
    };
    let render_target = create_render_target_and_set_camera(
        &mut camera,
        rendering::render_to_png::Resolution::Low
    );

    for i in 0..60 {
        clear_background(BACKGROUND_COLOR);

    let lines = get_l_system_lines(
        get_preset_l_system_configuration(PresetLSystemConfiguration::My3),
        0.1,
        8
    );

    for idx in 0..lines.len() {
        let line = &lines[idx];

        let color_percentage = idx as f32 / lines.len() as f32;

        // let mut final_color_percentage = color_percentage + runtime_configuration.color_percentage_offset;
        // while final_color_percentage > 1.0 { final_color_percentage -= 1.0; }

        // let line_color = interpolate_colors(BLUE, PINK, final_color_percentage);
        let line_color = get_rainbow_color(color_percentage);

        draw_line(
            line[0].x, line[0].y,
            line[1].x, line[1].y,
            2.0, line_color,
        );
    }

    set_default_camera();

    let image = render_target.texture.get_texture_data();
    image.export_png("./output/video_0/animation_0/frames/frame_00000.png");
    }
}
