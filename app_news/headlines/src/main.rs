mod headlines;

use std::sync::mpsc::{channel, sync_channel};
use std::thread;
use eframe::egui::{CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Rgba, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, Vec2, Visuals};
use eframe::epi::{App, Frame, Storage};
use eframe::{egui, NativeOptions, run_native};
use eframe::egui::TextStyle::Monospace;
use crate::headlines::{Headlines, Msg, NewsCardData};


impl App for Headlines {

    //Lllamado de una sola vez para configurar la app
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        let api_key= self.config.api_key.to_string();

        let (mut news_sender, news_receiver)= channel();
        let (app_sender, app_receiver)= sync_channel(1);

        self.app_sender = Some(app_sender);
        self.news_receiver = Some(news_receiver);

        thread::spawn(move ||{
            if !api_key.is_empty() {
                fetch_news(&api_key, &mut news_sender);
            }else {
                loop {
                    match app_receiver.recv() {
                        Ok(Msg::ApiKeySet(api_key))=>  {
                            fetch_news(&api_key,  &mut news_sender)
                        }
                        Err(e)=> {
                            tracing::error!("Error recibiendo mensaje {}", e);
                        }
                    }
                }
            }

        });

        self.configure_fonts(ctx);

    }

    //Este refresh se ejecutada 60 veces por segudo / 60fps
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {

        //Por defecto egui no escucha procesos si no tiene el focus
        //para evitar consumo de recursos, pero esto hace que
        //no se renderice sin el mouse encima, con la línea de abajo
        //se soluciona el proceso.
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        }else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        }else {
            self.preload_articles();

            CentralPanel::default()
                .show(ctx, |ui| {
                render_header(ui);
                self.render_news_cards(ctx, ui);
                render_footer(ui, ctx);
            });
        }
        self.render_top_panel(ctx, frame);
    }

    fn name(&self) -> &str {
        "Headlines"
    }

}

fn fetch_news(api_key: &str, news_sender:&mut std::sync::mpsc::Sender<NewsCardData>) {
    if let Ok(response) = newsapi::NewsAPI::new(&api_key).fetch() {
        let response_articles = response.articles();
        for article in response_articles {
            let news = NewsCardData {
                title: article.title().to_string(),
                url: article.url().to_string(),
                desc: article.desc()
                    .map(|val|{val.to_string()})
                    .unwrap_or("...".to_string())
            };

            if let Err(e) = news_sender.send(news){
                tracing::error!("Error sending news data: {}", e);
            }
        }
    }
}

fn render_footer(ui: &mut Ui, ctx: &CtxRef) {
    TopBottomPanel::bottom("bottom_panel")
        .min_height(70.)
        .max_height(70.)
        .show(ctx, |ui|{
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
    ui.add_space(headlines::PADDING*5.1);
    // let sep= Separator::default().spacing(10 as f32);
    // ui.add(sep);
}

fn main() {
    tracing_subscriber::fmt::init();

    let app= Headlines::new();
    let mut win_option= NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540 as f32, 700 as f32));
    win_option.resizable = false;
    run_native(Box::new(app), win_option);

}
