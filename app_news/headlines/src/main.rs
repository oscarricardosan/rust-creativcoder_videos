use eframe::egui::{CentralPanel, CtxRef, Rgba, ScrollArea, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};

struct Headlines {
    articles: Vec<NewsCardData>
}

impl Headlines {

    fn new() -> Headlines {

        let iter= (0..20).map(|index| NewsCardData{
            title: format!("Title {}", index),
            desc: format!("Desc {}", index),
            url: format!("https://example.com/{}", index)
        });

        Headlines{
            articles: iter.collect()
        }
    }

}

struct NewsCardData {
    title: String,
    desc: String,
    url: String
}

impl App for Headlines {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            //https://docs.rs/egui/0.15.0/egui/containers/struct.ScrollArea.html
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for new_card in &self.articles {
                    ui.label(&new_card.title);
                    ui.label(&new_card.desc);
                    ui.label(&new_card.url);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }

}

fn main() {
    let app= Headlines::new();
    let win_option= NativeOptions::default();
    run_native(Box::new(app), win_option);

}
