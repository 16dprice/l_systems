use macroquad::{color::WHITE, text::draw_text, time::{draw_fps, get_fps}};

pub struct CustomTelemetryData {
    pub theta: f32,
    pub delta_theta: f32,
    pub iterations: usize,
    pub get_lines_time: u128,
    pub for_loop_time: u128,
    pub color_percentage_offset: f32,
    pub num_lines: usize,
}

pub fn draw_custom_telemetry_data(custom_telemetry_data: CustomTelemetryData) {
    let fps = get_fps();
    draw_text(format!("FPS: {}", fps).as_str(), 20.0, 50.0, 30.0, WHITE);

    draw_text(format!("theta: {}", custom_telemetry_data.theta).as_str(), 20.0, 80.0, 30.0, WHITE);
    draw_text(format!("delta theta: {}", custom_telemetry_data.delta_theta).as_str(), 20.0, 110.0, 30.0, WHITE);
    draw_text(format!("iterations: {}", custom_telemetry_data.iterations).as_str(), 20.0, 140.0, 30.0, WHITE);

    draw_text(format!("lines time: {}", custom_telemetry_data.get_lines_time).as_str(), 20.0, 170.0, 30.0, WHITE);
    draw_text(format!("num lines: {}", custom_telemetry_data.num_lines).as_str(), 20.0, 200.0, 30.0, WHITE);
    draw_text(format!("for loop time: {}", custom_telemetry_data.for_loop_time).as_str(), 20.0, 230.0, 30.0, WHITE);
    draw_text(format!("color percentage offset: {}", custom_telemetry_data.color_percentage_offset).as_str(), 20.0, 260.0, 30.0, WHITE);
}