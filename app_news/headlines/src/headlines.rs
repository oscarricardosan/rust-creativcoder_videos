use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use serde::{Serialize, Deserialize};


pub const CONFY_FILE: &str= "headlines";

pub enum Msg {
    ExecuteFetch,
}

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String
}
impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self {
            dark_mode: Default::default(),
            api_key: String::new()
        }
    }
}

pub struct Headlines {
    pub articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub api_key_initialized: bool,
    pub news_receiver: Option<Receiver<NewsCardData>>,
    pub news_sender: Option<Arc<Mutex<Sender<NewsCardData>>>>,

    pub fetch_receiver: Option<Receiver<Msg>>,
    pub fetch_sender: Option<Arc<Mutex<SyncSender<Msg>>>>,
}

pub struct NewsCardData {
    pub title: String,
    pub desc: String,
    pub url: String
}

impl Headlines {

    pub fn new() -> Headlines {

        let config: HeadlinesConfig= confy::load(CONFY_FILE)
            .unwrap_or_default();

        Headlines{
            api_key_initialized: !config.api_key.is_empty(),
            config,
            articles: vec![],
            news_receiver: None,
            news_sender: None,
            fetch_receiver: None,
            fetch_sender: None
        }
    }

    pub fn fetch_news(&mut self) {
        if let Some(fetch_receiver) = &self.fetch_receiver {
            match fetch_receiver.try_recv(){
                Ok(Msg::ExecuteFetch)=> {
                    if let Ok(response) = newsapi::NewsAPI::new(&self.config.api_key).fetch() {
                        let response_articles = response.articles();
                        self.clear_news_cards();
                        for article in response_articles {
                            let news = NewsCardData {
                                title: article.title().to_string(),
                                url: article.url().to_string(),
                                desc: article.desc()
                                    .map(|val|{val.to_string()})
                                    .unwrap_or("...".to_string())
                            };

                            if let Some(news_sender) = &self.news_sender {
                                news_sender.lock().unwrap().send(news).unwrap();
                            }
                        }
                    }
                }
                Err(e)=> {
                    tracing::warn!("Error recibiendo orden para listar mensajes {}", e);
                }
            }
        }
    }

    pub fn load_articles(&mut self) {
        if let Some(news_receiver) = &self.news_receiver {
            match news_receiver.try_recv(){
                Ok(news_data)=> {
                    self.articles.push(news_data);
                },
                Err(e)=> {
                    tracing::warn!("Error recibiendo mensaje {}", e);
                }
            }
        }
    }
}
