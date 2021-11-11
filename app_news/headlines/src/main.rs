use eframe::egui::{CentralPanel, CtxRef, Rgba, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};

struct Headlines;

impl App for Headlines {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("article text");
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }

}

fn main() {
    let app= Headlines;
    let win_option= NativeOptions::default();
    run_native(Box::new(app), win_option);

}
