mod headlines;

use std::borrow::Cow;
use std::time::Duration;
use eframe::egui::{CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Rgba, ScrollArea, Separator, TextStyle, Ui, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};
use eframe::egui::Align::Min;
use crate::headlines::Headlines;

impl App for Headlines {

    //Lllamado de una sola vez para configurar la app
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }
    
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {

            self.render_news_cards(ui);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }

}

fn main() {
    let app= Headlines::new();
    let mut win_option= NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540 as f32, 960 as f32));
    run_native(Box::new(app), win_option);

}
