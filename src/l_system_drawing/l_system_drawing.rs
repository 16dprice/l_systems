use std::f32::consts::PI;

use crate::configurations::l_system_configurations::LSystemConfiguration;

use macroquad::prelude::*;

fn apply_configuration(
    l_system_configuration: LSystemConfiguration,
    iterations: usize,
) -> String {
    let mut l_string = String::from(l_system_configuration.axiom);

    for _ in 0..iterations {
        let mut string_parts: Vec<String> = vec![];

        // the bug here is that it's not keeping the stuff that's not a replacement rule
        for c in l_string.chars() {
            let mut handled_by_replacement_rule = false;
            for rule in &l_system_configuration.replacement_rules {
                if c == rule.from { string_parts.push(String::from(rule.to)); handled_by_replacement_rule = true; }
            }
            if !handled_by_replacement_rule { string_parts.push(String::from(c)); }
        }

        l_string = string_parts.join("");
    }

    return l_string;
}

struct LSystemStackValue {
    position: Vec2,
    dir: f32,
}

pub fn get_l_system_lines_from_string(l_string: &str, theta: f32) -> Vec<[Vec2; 2]> {
    let line_length = 800.0;

    let mut dir = PI / 2.0;
    let mut current_position = vec2(0.0, 0.0);
    let mut lines: Vec<[Vec2; 2]> = vec![];
    let mut l_system_stack: Vec<LSystemStackValue> = vec![];

    for c in l_string.chars() {
        if c == ' ' { continue; }

        // move forward
        if c == 'F' || c == 'G' {
            let new_x = current_position.x + f32::sqrt(line_length) * f32::cos(dir);
            let new_y = current_position.y - f32::sqrt(line_length) * f32::sin(dir);

            let new_position = vec2(new_x, new_y);

            lines.push([current_position.clone(), new_position.clone()]);

            current_position = vec2(new_x, new_y);
        }

        // turn right by angle theta
        if c == '-' { dir -= theta; }

        // turn left by angle theta
        if c == '+' { dir += theta; }

        // push onto stack
        if c == '[' { l_system_stack.push(LSystemStackValue { position: current_position, dir, }); }

        // pop off of stack and restore
        if c == ']' {
            let value = l_system_stack.pop();
            match value {
                None => { panic!("L System Stack doesn't have any values in it"); }
                Some(value) => {
                    current_position = value.position;
                    dir = value.dir;
                }
            }
        }
    }

    // ---------------- BEGIN SCALING AND TRANSFORM CODE ----------------
    let mut x_bounds = (0.0, 0.0);
    let mut y_bounds = (0.0, 0.0);

    for l in &lines {
        if l[0].x < x_bounds.0 { x_bounds.0 = l[0].x; }
        if l[0].x > x_bounds.1 { x_bounds.1 = l[0].x; }
        if l[1].x < x_bounds.0 { x_bounds.0 = l[1].x; }
        if l[1].x > x_bounds.1 { x_bounds.1 = l[1].x; }
        if l[0].y < y_bounds.0 { y_bounds.0 = l[0].y; }
        if l[0].y > y_bounds.1 { y_bounds.1 = l[0].y; }
        if l[1].y < y_bounds.0 { y_bounds.0 = l[1].y; }
        if l[1].y > y_bounds.1 { y_bounds.1 = l[1].y; }
    }

    let x_bounds_midpoint = (x_bounds.0 + x_bounds.1) / 2.0;
    let y_bounds_midpoint = (y_bounds.0 + y_bounds.1) / 2.0;
    let translation_vector = vec2(x_bounds_midpoint, y_bounds_midpoint);

    // first, translate points so that the center of the bounds is on the origin
    for l in &mut lines {
        l[0].x -= translation_vector.x;
        l[0].y -= translation_vector.y;

        l[1].x -= translation_vector.x;
        l[1].y -= translation_vector.y;
    }

    // second, scale points so that they fit in the window
    let x_scale_factor = screen_width() / (x_bounds.1 - x_bounds.0);
    let y_scale_factor = screen_height() / (y_bounds.1 - y_bounds.0);
    let scale_factor = f32::min(x_scale_factor, y_scale_factor);
 
    for l in &mut lines {
        l[0].x *= scale_factor;
        l[0].y *= scale_factor;

        l[1].x *= scale_factor;
        l[1].y *= scale_factor;
    }

    // ---------------- END SCALING AND TRANSFORM CODE ----------------

    return lines;
}

pub fn get_l_system_lines(l_system_configuration: LSystemConfiguration, theta: f32, iterations: usize) -> Vec<[Vec2; 2]> {
    let l_string = apply_configuration(l_system_configuration, iterations);
    return get_l_system_lines_from_string(&l_string, theta);
}
