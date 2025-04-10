mod configurations;
mod l_system_drawing;
mod ui;
mod rendering;
mod animation;
mod examples;

use core::num;
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
    telemetry::{draw_custom_telemetry_data, CustomTelemetryData}, text::draw_percentage_text_ex
};

use macroquad::prelude::*;

struct PercentageTextDrawingParams<'a, 'b> {
    text: &'b str,
    percentage: f32,
    x: f32,
    y: f32,
    font_size: u16,
    font: Option<&'a Font>,
    color: Color
}

async fn do_some_drawing() {
    let mut axiom_text_percentage = 0.0;
    let mut f_text_percentage = 0.0;
    let mut constants_text_percentage = 0.0;
    let mut plus_minus_text_percentage = 0.0;
    let mut production_rule_text_percentage = 0.0;
    let mut rule_text_percentage = 0.0;

    let font = load_ttf_font("./fonts/euler.otf").await.unwrap();

    let mut axiom_text_params = PercentageTextDrawingParams {
        text: "axiom: ",
        percentage: 0.0,
        x: 10.0,
        y: 50.0,
        font_size: 40,
        font: Some(&font),
        color: WHITE
    };

    loop {
        clear_background(BLACK);

        axiom_text_percentage += get_frame_time() / 2.0;
        if axiom_text_percentage > 1.0 { f_text_percentage += get_frame_time() / 0.2; }
        if f_text_percentage > 1.0 { constants_text_percentage += get_frame_time() / 2.0; }
        if constants_text_percentage > 1.0 { plus_minus_text_percentage += get_frame_time() / 0.5; }
        if plus_minus_text_percentage > 1.0 { production_rule_text_percentage += get_frame_time() / 2.0; }
        if production_rule_text_percentage > 1.0 { rule_text_percentage += get_frame_time() / 2.0; }

        let axiom_next_x_offset = draw_percentage_text_ex(
            "axiom: ",
            10.0, 50.0,
            axiom_text_percentage,
            TextParams {
                font_size: 40,
                font: Some(&font),
                color: WHITE,
                ..Default::default()
            }
        );
        draw_percentage_text_ex(
            "F",
            axiom_next_x_offset, 50.0,
            f_text_percentage,
            TextParams {
                font_size: 40,
                font: Some(&font),
                color: GREEN,
                ..Default::default()
            }
        );

        let constants_next_x_offset = draw_percentage_text_ex(
            "constants: ",
            10.0, 90.0,
            constants_text_percentage,
            TextParams {
                font_size: 40,
                font: Some(&font),
                color: WHITE,
                ..Default::default()
            }
        );
        draw_percentage_text_ex(
            "+, -",
            constants_next_x_offset, 90.0,
            plus_minus_text_percentage,
            TextParams {
                font_size: 40,
                font: Some(&font),
                color: RED,
                ..Default::default()
            }
        );

        let production_rule_next_x_offset = draw_percentage_text_ex(
            "production rule: ",
            10.0, 130.0,
            production_rule_text_percentage,
            TextParams {
                font_size: 40,
                font: Some(&font),
                color: WHITE,
                ..Default::default()
            }
        );
        draw_percentage_text_ex(
            "F -> F - F + F + F - F",
            production_rule_next_x_offset, 130.0,
            rule_text_percentage,
            TextParams {
                font_size: 40,
                font: Some(&font),
                color: BLUE,
                ..Default::default()
            }
        );
        
        next_frame().await;
    }
}

fn main_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    do_some_drawing().await;
}
