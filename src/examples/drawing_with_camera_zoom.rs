use macroquad::prelude::*;

pub async fn drawing_with_camera_zoom_example() {
    loop {
        let aspect_ratio = screen_width() / screen_height();
        let zoom_x = 1.0;
        let zoom_y = aspect_ratio;
        // a target of 0.0, 0.0 with no zoom defaults to a boundary box
        // of [-1, 1] in both the x and y axes
        // if zoom is zoom_x, zoom_y then the bounds are
        // [-1 / zoom_a, 1 / zoom_a] where zoom_a is zoom_x or zoom_y

        // if zoom_x = 2.0 / screen_width() and zoom_y = 2.0 / screen_height()
        // then the bounds will be [-screen_width() / 2.0, screen_width() / 2.0]
        // for x and similar for y
        set_camera(&Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(zoom_x, zoom_y),
            ..Default::default()
        });

        clear_background(BLACK);

        draw_line(
            -0.99 / zoom_x, -0.99 / zoom_y,
            0.99 / zoom_x, 0.99 / zoom_y,
            0.01, WHITE
        );
        

        next_frame().await;
    }
}