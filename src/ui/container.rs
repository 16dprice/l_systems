use std::f32::consts::PI;

use macroquad::prelude::*;

pub fn draw_container() {
    // thickness is also corner radius
    let thickness = 10.0;
    let width = 500.0;
    let height = 300.0;
    let x_offset = 0.0;
    let y_offset = 0.0;
    let resolution = 10;

    let vertices: [Vec2; 12] = [
        // idx 0
        vec2(
            x_offset,
            y_offset + thickness,
        ),
        // idx 1
        vec2(
            x_offset + thickness,
            y_offset,
        ),
        // idx 2
        vec2(
            x_offset + thickness,
            y_offset + thickness,
        ),
        // idx 3
        vec2(
            x_offset + thickness + width,
            y_offset,
        ),
        // idx 4
        vec2(
            x_offset + thickness + width,
            y_offset + thickness,
        ),
        // idx 5
        vec2(
            x_offset + thickness + width + thickness,
            y_offset + thickness,
        ),
        // idx 6
        vec2(
            x_offset + thickness + width,
            y_offset + thickness + height,
        ),
        // idx 7
        vec2(
            x_offset + thickness + width + thickness,
            y_offset + thickness + height,
        ),
        // idx 8
        vec2(
            x_offset + thickness + width,
            y_offset + thickness + height + thickness,
        ),
        // idx 9
        vec2(
            x_offset,
            y_offset + thickness + height,
        ),
        // idx 10
        vec2(
            x_offset + thickness,
            y_offset + thickness + height,
        ),
        // idx 11
        vec2(
            x_offset + thickness,
            y_offset + thickness + height + thickness,
        ),
    ];
    
    let outline_indices: [(usize, usize, usize); 8] = [
        // top side
        (1, 2, 3),
        (2, 3, 4),
        // right side
        (4, 5, 6),
        (5, 6, 7),
        // bottom side
        (6, 10, 11),
        (6, 8, 11),
        // left side
        (0, 2, 9),
        (2, 9, 10),
    ];

    let center_indices: [(usize, usize, usize); 2] = [
        (2, 4, 10),
        (4, 6, 10)
    ];

    for (idx0, idx1, idx2) in outline_indices {
        draw_triangle(
            vertices[idx0],
            vertices[idx1],
            vertices[idx2],
            RED
        );
    }

    for (idx0, idx1, idx2) in center_indices {
        draw_triangle(
            vertices[idx0],
            vertices[idx1],
            vertices[idx2],
            GRAY
        );
    }

    let draw_corner_points = |
        theta_0: f32,
        theta_1: f32,
        start_corner: Vec2,
        end_corner: Vec2,
        inner_corner_vertex: Vec2,
    | {
        let mut top_left_corner_points: Vec<Vec2> = vec![start_corner];
        let theta_delta = (theta_1 - theta_0) / resolution as f32;
        for i in 1..resolution {
            let alpha = theta_0 + (i as f32) * theta_delta;

            let x = inner_corner_vertex.x + thickness * f32::cos(alpha);
            let y = inner_corner_vertex.y - thickness * f32::sin(alpha);

            top_left_corner_points.push(
                vec2(
                    x, y
                )
            );
        }
        top_left_corner_points.push(end_corner);

        for i in 1..top_left_corner_points.len() {
            draw_triangle(
                inner_corner_vertex,
                top_left_corner_points[i],
                top_left_corner_points[i - 1],
                RED,
            );
        }
    };

    // top left corner
    draw_corner_points(
        PI / 2.0,
        PI,
        vertices[1],
        vertices[0],
        vertices[2]
    );

    // top right corner
    draw_corner_points(
        0.0,
        PI / 2.0,
        vertices[5],
        vertices[3],
        vertices[4]
    );

    // bottom right corner
    draw_corner_points(
        2.0 * PI,
        3.0 * PI / 2.0,
        vertices[7],
        vertices[8],
        vertices[6]
    );

    // bottom left corner
    draw_corner_points(
        3.0 * PI / 2.0,
        PI,
        vertices[11],
        vertices[9],
        vertices[10]
    );
}
