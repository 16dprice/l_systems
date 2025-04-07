use macroquad::math::Vec2;

pub struct RuntimeConfiguration {
    // Drawing Config
    pub theta: f32,
    pub delta_theta: f32,
    pub iterations: usize,

    // Color Config
    pub color_percentage_offset: f32,

    // Camera Config
    pub camera_target: Vec2,
    pub camera_zoom: Vec2,
    pub camera_rotation: f32,

    // Telemetry
    pub show_telemetry: bool,
}
