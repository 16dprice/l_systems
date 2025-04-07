use crate::configurations::runtime_configuration::RuntimeConfiguration;

use macroquad::{input::{is_key_down, is_key_pressed, KeyCode}, time::get_fps};

pub fn handle_input(runtime_configuration: &mut RuntimeConfiguration) {
    let current_fps = get_fps() as f32;

    if is_key_down(KeyCode::Right) { runtime_configuration.theta += runtime_configuration.delta_theta / current_fps; }
    if is_key_down(KeyCode::Left) { runtime_configuration.theta -= runtime_configuration.delta_theta / current_fps; }
    if is_key_down(KeyCode::Up) { runtime_configuration.delta_theta += 0.006 / current_fps; }
    if is_key_down(KeyCode::Down) { runtime_configuration.delta_theta -= 0.006 / current_fps; }

    if is_key_pressed(KeyCode::RightBracket) { runtime_configuration.iterations += 1; }
    if is_key_pressed(KeyCode::LeftBracket) { if runtime_configuration.iterations > 1 { runtime_configuration.iterations -= 1; } }

    if is_key_down(KeyCode::W) { runtime_configuration.camera_target.y -= 300.0 / current_fps; }
    if is_key_down(KeyCode::S) { runtime_configuration.camera_target.y += 300.0 / current_fps; }
    if is_key_down(KeyCode::A) { runtime_configuration.camera_target.x -= 300.0 / current_fps; }
    if is_key_down(KeyCode::D) { runtime_configuration.camera_target.x += 300.0 / current_fps; }
    if is_key_down(KeyCode::I) { runtime_configuration.camera_zoom.x += 0.006 / current_fps; runtime_configuration.camera_zoom.y += 0.006 / current_fps; }
    if is_key_down(KeyCode::O) { runtime_configuration.camera_zoom.x -= 0.006 / current_fps; runtime_configuration.camera_zoom.y -= 0.006 / current_fps; }
    if is_key_down(KeyCode::E) { runtime_configuration.camera_rotation -= 60.0 / current_fps; }
    if is_key_down(KeyCode::R) { runtime_configuration.camera_rotation += 60.0 / current_fps; }

    if is_key_pressed(KeyCode::T) { runtime_configuration.show_telemetry = !runtime_configuration.show_telemetry; }
}
