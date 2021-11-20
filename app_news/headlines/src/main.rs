mod headlines;
mod headlines_app;
mod hadlines_render;

use eframe::egui::{Vec2};
use eframe::{NativeOptions, run_native};
use crate::headlines::{Headlines, Msg};

fn main() {
    tracing_subscriber::fmt::init();

    let app= Headlines::new();
    let mut win_option= NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540 as f32, 700 as f32));
    win_option.resizable = false;
    run_native(Box::new(app), win_option);

}
