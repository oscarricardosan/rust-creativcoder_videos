mod headlines;

use eframe::egui::{CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Rgba, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};
use eframe::egui::TextStyle::Monospace;
use crate::headlines::Headlines;

impl App for Headlines {

    //Lllamado de una sola vez para configurar la app
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }
    
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            self.render_news_cards(ui);
            render_footer(ui, ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }

}

fn render_footer(ui: &mut Ui, ctx: &CtxRef) {
    TopBottomPanel::bottom("id_source").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            ui.add_space(10.);
            ui.add(Label::new("API fuente: newsapi.org").monospace());
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
                    .text("Hecho con egui")
                    .text_style(Monospace)
            );
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
                    .text("oscarricardosan/rust-creativcoder_videos")
                    .text_style(Monospace)
            );
            ui.add_space(10.);
        });
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Headlines");
    });
    ui.add_space(headlines::PADDING);
    let sep= Separator::default().spacing(10 as f32);
    ui.add(sep);
}

fn main() {
    let app= Headlines::new();
    let mut win_option= NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540 as f32, 960 as f32));
    run_native(Box::new(app), win_option);

}
