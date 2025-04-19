use std::sync::{Arc, Mutex};
use std::{f32::consts::PI, time::Duration};

use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use indicatif::ProgressBar;
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::WHITE, math::vec2, text::load_ttf_font, texture::{get_screen_data, Image}, window::{clear_background, next_frame, screen_height}};

use crate::png::write_png;
use crate::{configurations::l_system_configurations::{get_preset_l_system_configuration, PresetLSystemConfiguration}, l_system::{LSystem, ScaleParams}, rendering::render_to_png::{create_render_target_and_set_camera, get_width_and_height_from_resolution, Resolution}, text::DrawableText, BACKGROUND_COLOR};

const ANIMATION_NUM_IO_THREADS: usize = 6;

pub async fn render_animation_0(
    start_frame: u64,
    output_dir: &str,
    resolution: &Resolution
) -> u64 {
    let (width, height) = get_width_and_height_from_resolution(&resolution);

    let desired_fps = 60;
    let total_seconds_of_video = 35;

    let mut camera = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(2.0 / width as f32, 2.0 / height as f32),
        // rotation: 180.0,
        ..Default::default()
    };

    let zoom_factor = 0.7;
    let y = 600.0 / zoom_factor;

    let start_l_system = LSystem {
        iterations: 6,
        l_system_configuration: &get_preset_l_system_configuration(PresetLSystemConfiguration::My3),
        theta: 0.0,
        start_position: vec2(0.0, 0.0),
        scale_params: ScaleParams {
            translate: true,
            scale: true,
            width: width as f32,
            height: height as f32,
        }
    };

    let end_l_system = LSystem {
        iterations: 6,
        l_system_configuration: &get_preset_l_system_configuration(PresetLSystemConfiguration::My3),
        theta: PI / 5.0,
        start_position: vec2(0.0, 0.0),
        scale_params: ScaleParams {
            translate: true,
            scale: true,
            width: width as f32,
            height: height as f32,
        }
    };

    let mut handles = Vec::<(JoinHandle<()>, Sender<Option<(Image, String)>>)>::new();

    let progress_bar = Arc::new(Mutex::new(ProgressBar::new(desired_fps * total_seconds_of_video)));
    for i in 0..ANIMATION_NUM_IO_THREADS {
        let (tx, rx) = mpsc::channel::<Option<(Image, String)>>();
        let progress_bar = Arc::clone(&progress_bar);
        
        let handle = thread::spawn(move || {
            let mut is_running = true;
            while is_running {
                match rx.try_recv() {
                    Ok(val) => {
                        match val {
                            Some((image, path)) => {
                                progress_bar.lock().unwrap().inc(1);
                                write_png(&image.bytes, image.width, image.height, &path);
                            }
                            None => { is_running = false; }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        handles.push((handle, tx));
    }

    for frame in start_frame..(start_frame + desired_fps * total_seconds_of_video) {
        clear_background(BACKGROUND_COLOR);
        set_camera(&camera);

        let percentage = (frame + 1) as f32 / (start_frame + desired_fps * total_seconds_of_video) as f32;

        LSystem::animate_between(&start_l_system, &end_l_system, percentage);

        let image = get_screen_data();
        let current_handle = frame as usize % ANIMATION_NUM_IO_THREADS;
        handles[current_handle].1.send(Some((image, format!("{}/frame_{:>05}.png", output_dir, frame)))).unwrap();

        next_frame().await;
    }

    for h in &handles {
        h.1.send(None);
    }
    
    for h in handles {
        h.0.join();
    }

    return start_frame + desired_fps * total_seconds_of_video;
}

pub async fn render_animation_1(
    start_frame: u64,
    output_dir: &str,
    resolution: &Resolution
) -> u64 {
    let (width, height) = get_width_and_height_from_resolution(&resolution);
    let font = load_ttf_font("./fonts/euler.otf").await.unwrap();

    let mut drawable_text = DrawableText {
        lines: vec![
            "\u{1D415} = { \u{1D405}, +, \u{2212}, [, ] }",
            "\u{1D40E} = \u{1D405}", // TODO: would be great to get actual lower case omega here
            "\u{1D40F} = { \u{1D405} \u{2192} \u{1D405} [ \u{2212} \u{1D405} \u{2212} \u{1D405} ] [ + \u{1D405} + \u{1D405} ] \u{1D405} }"
        ],
        x: 300.0,
        y: 200.0,
        max_width: 100.0,
        max_font_size: 20,
        vertical_gap: 25.0,
        percentage_to_draw: 0.0,

        color: WHITE,
        font: Some(&font),
        font_scale: 1.0,
    };

    let desired_fps = 60;
    let total_seconds_of_video = 6;

    let mut handles = Vec::<
        (
            JoinHandle<()>,
            Sender<Option<(Image, String)>>
        )
    >::new();

    let progress_bar = Arc::new(Mutex::new(ProgressBar::new(desired_fps * total_seconds_of_video)));
    for i in 0..ANIMATION_NUM_IO_THREADS {
        let (tx, rx) = mpsc::channel::<Option<(Image, String)>>();
        let progress_bar = Arc::clone(&progress_bar);
        let handle = thread::spawn(move || {
            let mut is_running = true;
            while is_running {
                match rx.try_recv() {
                    Ok(val) => {
                        match val {
                            Some((image, path)) => {
                                progress_bar.lock().unwrap().inc(1);
                                image.export_png(&path);
                            }
                            None => { is_running = false; }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        handles.push((handle, tx));
    }

    set_default_camera();
    for frame in start_frame..(start_frame + desired_fps * total_seconds_of_video) {
        clear_background(BACKGROUND_COLOR);

        drawable_text.percentage_to_draw = (frame + 1) as f32 / (start_frame + desired_fps * total_seconds_of_video) as f32;
        drawable_text.draw_box_text_ex();

        let image = get_screen_data();
        let current_handle = frame as usize % ANIMATION_NUM_IO_THREADS;
        handles[current_handle].1.send(Some((image, format!("{}/frame_{:>05}.png", output_dir, frame)))).unwrap();

        next_frame().await;
    }

    for h in &handles {
        h.1.send(None);
    }
    
    for h in handles {
        h.0.join();
    }

    return start_frame + desired_fps * total_seconds_of_video;
}
