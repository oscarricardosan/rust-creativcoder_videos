mod headlines;

use eframe::egui::{CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Rgba, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, Vec2, Visuals};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};
use eframe::egui::TextStyle::Monospace;
use crate::headlines::{Headlines, NewsCardData};


fn fetch_news(api_key: &str, articles: &mut Vec<NewsCardData> ) {
    if let Ok(response) = newsapi::NewsAPI::new(api_key).fetch() {
        let response_articles = response.articles();
        for article in response_articles {
            let news = NewsCardData {
                title: article.title().to_string(),
                url: article.url().to_string(),
                desc: article.desc()
                    .map(|val|{val.to_string()})
                    .unwrap_or("...".to_string())
            };
            articles.push(news);
        }
    }
}

impl App for Headlines {

    //Lllamado de una sola vez para configurar la app
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        fetch_news(&self.config.api_key, &mut self.articles);
        self.configure_fonts(ctx);
    }

    //Este refresh se ejecutada 60 veces por segudo / 60fps
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        }else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        }else {
            CentralPanel::default().show(ctx, |ui| {
                render_header(ui);
                self.render_news_cards(ui);
                render_footer(ui, ctx);
            });
        }
        self.render_top_panel(ctx, frame);
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
    tracing_subscriber::fmt::init();

    let app= Headlines::new();
    let mut win_option= NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540 as f32, 960 as f32));
    run_native(Box::new(app), win_option);

}
