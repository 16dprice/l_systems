#![allow(unused)]
mod configurations;
mod examples;
mod png;
mod l_system;
mod rendering;
mod text;
mod ui;
mod videos;

use core::num;
use std::{f32::consts::PI, time::{Duration, SystemTime}};

use configurations::{
    l_system_configurations::{get_preset_l_system_configuration, LSystemConfiguration, PresetLSystemConfiguration},
    runtime_configuration::RuntimeConfiguration
};
use indicatif::ProgressBar;
use l_system::{LSystem, ScaleParams};
use rendering::{
    render_to_png::{
        create_render_target_and_set_camera,
        get_width_and_height_from_resolution,
        render_lines_to_png,
        Resolution
    },
    render_to_screen::{
        render_lines_to_screen,render_telemetry_to_screen
    }
};
use text::DrawableText;
use ui::{
    colors::{get_rainbow_color, interpolate_colors},
    container::draw_container,
    input::handle_input,
    telemetry::{draw_custom_telemetry_data, CustomTelemetryData}, text::{draw_box_text_ex, draw_percentage_text_ex, BoxTextParams}
};

use macroquad::prelude::*;
use videos::video_0::{render_animation_0, render_animation_1};

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
    // let next_start_frame = render_animation_0(0, "./output/video_0/animation_0/frames", &Resolution::Ultra).await;
    let next_start_frame = render_animation_1(0, "./output/video_0/animation_1/frames").await;
}
