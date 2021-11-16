use std::borrow::Cow;
use eframe::egui;
use eframe::egui::{Button, Color32, CtxRef, FontDefinitions, FontFamily, Frame, Hyperlink, Label, Layout, ScrollArea, Separator, TextStyle, Ui};
use eframe::egui::Align::Min;
use crate::TopBottomPanel;
use serde::{Serialize, Deserialize};

pub const PADDING: f32= 3.0;
const WHITE: Color32= Color32::from_rgb(255, 255, 255);
const BLACK: Color32= Color32::from_rgb(0, 0, 0);
const CYAN: Color32= Color32::from_rgb(0, 255, 255);
const RED: Color32= Color32::from_rgb(255, 0, 0);

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    pub dark_mode: bool
}
impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self {
            dark_mode: Default::default()
        }
    }
}
impl HeadlinesConfig {
    fn new() -> Self {
        Self{
            dark_mode: true
        }
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String
}

impl Headlines {

    pub fn new() -> Headlines {

        let iter= (0..20).map(|index| NewsCardData{
            title: format!("Title {}", index),
            desc: format!("Desc {}", index),
            url: format!("https://example.com/{}", index)
        });

        let config: HeadlinesConfig= confy::load("setup_headlines")
            .unwrap_or_default();

        Headlines{
            articles: iter.collect(),
            config
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();

        //Agregamos nuestra fuente personalizada
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf"))
        );
        //Fuente de Cabecera proporcional de 35 puntos
        font_def.family_and_size.insert(
            TextStyle::Heading,
            (
                FontFamily::Proportional,
                28.
            )
        );
        //Fuente de Body proporcional de 35 puntos
        font_def.family_and_size.insert(
            TextStyle::Body,
            (
                FontFamily::Proportional,
                15.
            )
        );

        //Datle prioridad a nuestra fuente en el Front
        font_def.fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);

    }
    pub fn render_news_cards(&self, ui: &mut Ui) {

        //https://docs.rs/egui/0.15.0/egui/containers/struct.ScrollArea.html
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for news_card in &self.articles {
                ui.add_space(PADDING);
                //Render title
                let title= format!("► {}", news_card.title);
                if self.config.dark_mode {
                    ui.colored_label(WHITE, title);
                }else{
                    ui.colored_label(BLACK, title);
                }
                //Render desc
                ui.add_space(PADDING);
                let desc= Label::new(&news_card.desc)
                    .text_style(TextStyle::Button);
                ui.add(desc);

                //Render hyperlynks
                if self.config.dark_mode {
                    ui.style_mut().visuals.hyperlink_color = CYAN;
                } else {
                    ui.style_mut().visuals.hyperlink_color = RED;
                }
                ui.add_space(PADDING);

                let mut layout = Layout::right_to_left();
                layout = layout.with_cross_align(Min);
                ui.with_layout(layout, |ui| {
                    ui.add(Hyperlink::new(&news_card.url).text("Leer mas ⤴"));
                });
                ui.add_space(PADDING);
                ui.add(Separator::default());
            }
        });
    }

    pub fn render_top_panel(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.);
            egui::menu::bar(ui, |ui|{
                //Logo
                ui.with_layout(
                    Layout::left_to_right(), |ui|{
                        ui.add(
                            Label::new("📕").text_style(egui::TextStyle::Heading)
                        );
                    }
                );
                //Controls
                ui.with_layout(
                    Layout::right_to_left(), |ui|{
                        let close_btn= ui.add(
                            Button::new("❌").text_style(TextStyle::Body)
                        );
                        if close_btn.clicked() {
                            frame.quit();
                        }

                        let refresh_btn= ui.add(
                            Button::new("🔄").text_style(TextStyle::Body)
                        );
                        let theme_btn= ui.add(
                            Button::new({
                                if self.config.dark_mode {
                                    "☀"
                                }else{
                                    "🌜"
                                }
                            }).text_style(TextStyle::Body)
                        );
                        if theme_btn.clicked() {
                            self.config.dark_mode= !self.config.dark_mode;
                        }
                    }
                );

            });
            ui.add_space(5.);
        });
    }

}
