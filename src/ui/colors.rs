use macroquad::color::Color;

const RED:          Color = Color { r: 255.0 / 255.0, g:   0.0 / 255.0, b:   0.0 / 255.0, a: 1.0 };
const ORANGE:       Color = Color { r: 255.0 / 255.0, g: 127.0 / 255.0, b:   0.0 / 255.0, a: 1.0 };
const YELLOW:       Color = Color { r: 255.0 / 255.0, g: 255.0 / 255.0, b:   0.0 / 255.0, a: 1.0 };
const GREEN_1:      Color = Color { r: 127.0 / 255.0, g: 255.0 / 255.0, b:   0.0 / 255.0, a: 1.0 };
const GREEN_2:      Color = Color { r:   0.0 / 255.0, g: 255.0 / 255.0, b:   0.0 / 255.0, a: 1.0 };
const GREEN_3:      Color = Color { r:   0.0 / 255.0, g: 255.0 / 255.0, b: 127.0 / 255.0, a: 1.0 };
const CYAN:         Color = Color { r:   0.0 / 255.0, g: 255.0 / 255.0, b: 255.0 / 255.0, a: 1.0 };
const LIGHT_BLUE:   Color = Color { r:   0.0 / 255.0, g: 127.0 / 255.0, b: 255.0 / 255.0, a: 1.0 };
const BLUE:         Color = Color { r:   0.0 / 255.0, g:   0.0 / 255.0, b: 255.0 / 255.0, a: 1.0 };
const PURPLE:       Color = Color { r: 127.0 / 255.0, g:   0.0 / 255.0, b: 255.0 / 255.0, a: 1.0 };
const MAGENTA:      Color = Color { r: 255.0 / 255.0, g:   0.0 / 255.0, b: 255.0 / 255.0, a: 1.0 };
const RED_PINK:     Color = Color { r: 255.0 / 255.0, g:   0.0 / 255.0, b: 127.0 / 255.0, a: 1.0 };

pub fn get_rainbow_color(percentage: f32) -> Color {
    let rainbow_colors: [Color; 12] = [
        RED,
        ORANGE,
        YELLOW,
        GREEN_1,
        GREEN_2,
        GREEN_3,
        CYAN,
        LIGHT_BLUE,
        BLUE,
        PURPLE,
        MAGENTA,
        RED_PINK
    ];

    let idx = (percentage * 11.0) as usize;
    let color_percentage = (percentage * 11.0) - idx as f32;

    let start = rainbow_colors[idx];
    let end = rainbow_colors[(idx + 1) % 12];

    return Color {
        r: (1.0 - color_percentage) * start.r + color_percentage * end.r,
        g: (1.0 - color_percentage) * start.g + color_percentage * end.g,
        b: (1.0 - color_percentage) * start.b + color_percentage * end.b,
        a: (1.0 - color_percentage) * start.a + color_percentage * end.a,
    }
}

pub fn interpolate_colors(start: Color, end: Color, percentage: f32) -> Color {
    let r = (1.0 - percentage) * start.r + percentage * end.r;
    let g = (1.0 - percentage) * start.g + percentage * end.g;
    let b = (1.0 - percentage) * start.b + percentage * end.b;
    let a = (1.0 - percentage) * start.a + percentage * end.a;

    return Color { r, g, b, a };
}
