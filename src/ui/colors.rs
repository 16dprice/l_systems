use macroquad::color::Color;

pub fn get_rainbow_color(percentage: f32) -> Color {
    let rainbow_colors: [Color; 12] = [
        Color { r: 255.0 / 255.0, g:   0.0 / 255.0, b:   0.0 / 255.0, a: 1.0 }, // red
        Color { r: 255.0 / 255.0, g: 127.0 / 255.0, b:   0.0 / 255.0, a: 1.0 }, // orange
        Color { r: 255.0 / 255.0, g: 255.0 / 255.0, b:   0.0 / 255.0, a: 1.0 }, // yellow
        Color { r: 127.0 / 255.0, g: 255.0 / 255.0, b:   0.0 / 255.0, a: 1.0 }, // green 1
        Color { r:   0.0 / 255.0, g: 255.0 / 255.0, b:   0.0 / 255.0, a: 1.0 }, // green 2
        Color { r:   0.0 / 255.0, g: 255.0 / 255.0, b: 127.0 / 255.0, a: 1.0 }, // green 3
        Color { r:   0.0 / 255.0, g: 255.0 / 255.0, b: 255.0 / 255.0, a: 1.0 }, // cyan
        Color { r:   0.0 / 255.0, g: 127.0 / 255.0, b: 255.0 / 255.0, a: 1.0 }, // light blue
        Color { r:   0.0 / 255.0, g:   0.0 / 255.0, b: 255.0 / 255.0, a: 1.0 }, // blue
        Color { r: 127.0 / 255.0, g:   0.0 / 255.0, b: 255.0 / 255.0, a: 1.0 }, // purple
        Color { r: 255.0 / 255.0, g:   0.0 / 255.0, b: 255.0 / 255.0, a: 1.0 }, // magenta
        Color { r: 255.0 / 255.0, g:   0.0 / 255.0, b: 127.0 / 255.0, a: 1.0 }, // red-pink
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
