mod headlines;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, sync_channel};
use eframe::egui::{CentralPanel, CtxRef, Hyperlink, Label, TopBottomPanel, Ui, Vec2, Visuals};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};
use eframe::egui::TextStyle::Monospace;
use crate::headlines::{Headlines, Msg};


impl App for Headlines {

    //Lllamado de una sola vez para configurar la app
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {

        let (news_sender, news_receiver)= channel();
        let (fetch_sender, fetch_receiver)= sync_channel(1);

        self.news_receiver = Some(news_receiver);
        self.news_sender = Some(Arc::new(Mutex::new(news_sender)));

        self.fetch_receiver = Some(fetch_receiver);
        let fetch_sender = Arc::new(Mutex::new(fetch_sender));
        self.fetch_sender = Some(fetch_sender.clone());

        if self.api_key_initialized {
            fetch_sender.lock().unwrap().send(Msg::ExecuteFetch);
        }

        self.configure_fonts(ctx);

    }

    //Este refresh se ejecuta 60 veces por segudo / 60fps
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
            self.load_articles();

            CentralPanel::default()
                .show(ctx, |ui| {
                render_header(ui);
                self.render_news_cards(ctx, ui);
                render_footer(ui, ctx);
            });
        }
        self.render_top_panel(ctx, frame);
        self.fetch_news();
    }

    fn name(&self) -> &str {
        "Headlines"
    }

}

fn render_footer(_ui: &mut Ui, ctx: &CtxRef) {
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
