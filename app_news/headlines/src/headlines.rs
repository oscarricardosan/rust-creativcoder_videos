use std::borrow::Cow;
use eframe::egui::{Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, ScrollArea, Separator, TextStyle, Ui};
use eframe::egui::Align::Min;

const PADDING: f32= 3.0;
const WHITE: Color32= Color32::from_rgb(255, 255, 255);
const CYAN: Color32= Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>
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

        Headlines{
            articles: iter.collect()
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
                ui.colored_label(WHITE, title);
                //Render desc
                ui.add_space(PADDING);
                let desc= Label::new(&news_card.desc)
                    .text_style(TextStyle::Button);
                ui.add(desc);

                //Render hyperlynks
                ui.style_mut().visuals.hyperlink_color= CYAN;
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

}
