use crate::configurations::runtime_configuration::RuntimeConfiguration;

use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

pub fn handle_input(runtime_configuration: &mut RuntimeConfiguration) {
    if is_key_down(KeyCode::Right) { runtime_configuration.theta += runtime_configuration.delta_theta; }
    if is_key_down(KeyCode::Left) { runtime_configuration.theta -= runtime_configuration.delta_theta; }
    if is_key_down(KeyCode::Up) { runtime_configuration.delta_theta += 0.00001; }
    if is_key_down(KeyCode::Down) { runtime_configuration.delta_theta -= 0.00001; }

    if is_key_pressed(KeyCode::RightBracket) { runtime_configuration.iterations += 1; }
    if is_key_pressed(KeyCode::LeftBracket) { if runtime_configuration.iterations > 1 { runtime_configuration.iterations -= 1; } }

    if is_key_down(KeyCode::W) { runtime_configuration.camera_target.y -= 5.0; }
    if is_key_down(KeyCode::S) { runtime_configuration.camera_target.y += 5.0; }
    if is_key_down(KeyCode::A) { runtime_configuration.camera_target.x -= 5.0; }
    if is_key_down(KeyCode::D) { runtime_configuration.camera_target.x += 5.0; }
    if is_key_down(KeyCode::I) { runtime_configuration.camera_zoom.x += 0.0001; runtime_configuration.camera_zoom.y += 0.0001; }
    if is_key_down(KeyCode::O) { runtime_configuration.camera_zoom.x -= 0.0001; runtime_configuration.camera_zoom.y -= 0.0001; }
    if is_key_down(KeyCode::E) { runtime_configuration.camera_rotation -= 1.0; }
    if is_key_down(KeyCode::R) { runtime_configuration.camera_rotation += 1.0; }
}
