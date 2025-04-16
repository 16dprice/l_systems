use std::f32::consts::PI;

use macroquad::{color::{Color, BROWN, GREEN}, math::{vec2, Vec2}, prelude::warn, shapes::draw_line};

use crate::{configurations::l_system_configurations::LSystemConfiguration, ui::colors::get_rainbow_color};

pub struct ScaleParams {
    pub translate: bool,
    pub scale: bool,
    pub width: f32,
    pub height: f32,
}

pub struct LSystem<'a> {
    pub iterations: usize,
    pub l_system_configuration: &'a LSystemConfiguration<'a>,
    pub theta: f32,
    pub start_position: Vec2,
    pub scale_params: ScaleParams
}

struct LSystemStackValue {
    position: Vec2,
    dir: f32,
}

impl<'a> LSystem<'a> {
    pub fn animate(
        start: &LSystem,
        end: &LSystem,
        percentage: f32,
    ) {
        if start.iterations != end.iterations {
            warn!(
                "{} != {}. Start frame iterations ({}) will be chosen",
                start.iterations,
                end.iterations,
                start.iterations
            );
        }

        if percentage < 0.0 {
            start.draw();
            return;
        }

        if percentage > 1.0 {
            end.draw();
            return;
        }

        fn lerp(start: f32, end: f32, perc: f32) -> f32 {
            (1.0 - perc) * start + perc * end
        }

        let current = LSystem {
            iterations: start.iterations,
            l_system_configuration: start.l_system_configuration,
            start_position: vec2(
                lerp(start.start_position.x, end.start_position.x, percentage),
                lerp(start.start_position.y, end.start_position.y, percentage),
            ),
            theta: lerp(start.theta, end.theta, percentage),
            scale_params: ScaleParams {
                translate: start.scale_params.translate,
                scale: start.scale_params.scale,
                width: lerp(start.scale_params.width, end.scale_params.width, percentage),
                height: lerp(start.scale_params.height, end.scale_params.height, percentage)
            }
        };
        current.draw();
    }

    pub fn draw(&self) {
        let lines = self.get_lines();
        for idx in 0..lines.len() {
            let line = &lines[idx];
            let color_percentage = idx as f32 / lines.len() as f32;
            let line_color = get_rainbow_color(color_percentage);

            // fn lerp(start: f32, end: f32, perc: f32) -> f32 { (1.0 - perc) * start + perc * end }
            // let line_color = Color {
            //     r: lerp(GREEN.r, BROWN.r, color_percentage),
            //     g: lerp(GREEN.g, BROWN.g, color_percentage),
            //     b: lerp(GREEN.b, BROWN.b, color_percentage),
            //     a: 1.0
            // };
    
            draw_line(
                line[0].x, line[0].y,
                line[1].x, line[1].y,
                2.0, line_color,
            );
        }
    }

    pub fn get_lines(&self) -> Vec<[Vec2; 2]> {
        let l_string = LSystem::apply_configuration(&self.l_system_configuration, self.iterations);
        let line_length = 200.0;
    
        let mut dir = PI / 2.0;
        let mut current_position = self.start_position;
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
            if c == '-' { dir -= self.theta; }
    
            // turn left by angle theta
            if c == '+' { dir += self.theta; }
    
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

        if self.scale_params.translate {
            let translation_vector = vec2(x_bounds_midpoint, y_bounds_midpoint);
        
            // first, translate points so that the center of the bounds is on the origin
            for l in &mut lines {
                l[0].x -= translation_vector.x;
                l[0].y -= translation_vector.y;
        
                l[1].x -= translation_vector.x;
                l[1].y -= translation_vector.y;
            }
        }
    
        if self.scale_params.scale {
            // second, scale points so that they fit in the window
            let x_scale_factor = self.scale_params.width / (x_bounds.1 - x_bounds.0);
            let y_scale_factor = self.scale_params.height / (y_bounds.1 - y_bounds.0);
            let scale_factor = f32::min(x_scale_factor, y_scale_factor);
        
            for l in &mut lines {
                l[0].x *= scale_factor;
                l[0].y *= scale_factor;
        
                l[1].x *= scale_factor;
                l[1].y *= scale_factor;
            }
        }
    
        // ---------------- END SCALING AND TRANSFORM CODE ----------------
    
        return lines;
    }

    pub fn apply_configuration(
        l_system_configuration: &LSystemConfiguration,
        iterations: usize,
    ) -> String {
        let mut l_string = String::from(l_system_configuration.axiom);
    
        for _ in 0..iterations {
            let mut string_parts: Vec<String> = vec![];
    
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
}
