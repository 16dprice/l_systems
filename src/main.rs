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
use rendering::{render_to_png::{create_render_target_and_set_camera, render_lines_to_png, Resolution}, render_to_screen::{render_lines_to_screen, render_telemetry_to_screen}};
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

/**
 * Want multiple things
 * 1. function that will draw text in a bounding box and will scale text to fit in the box
 *      i. should this func take in desired number of lines?
 * 2. function that will draw text one character at a time over some certain period of time
 * 3. function that does both
 * 
 * Task
 * 1. Build out first function that draws in bounding box
 * 2. Look at draw_percentage_text_ex and abstrac the functionality that can be abstracted
 *      for getting the chars and the alpha values for the chars
 */

#[macroquad::main(main_conf)]
async fn main() {
    let font = load_ttf_font("./fonts/euler.otf").await.unwrap();

    loop {
        draw_box_text_ex(
            vec!["Some text that should go in a box", "some more text gin a box", "Some more text in a b"], 
            10.0, 
            100.0,
            BoxTextParams {
                max_font_size: 40,
                max_width: 500.0,
                vertical_gap: 10.0,
                text_params: TextParams {
                    font: Some(&font),
                    color: WHITE,
                    ..Default::default()
                }
            }
        );
        // return;
        next_frame().await;
    }
}
