mod theme;

use std::error::Error;
use newsapi::{Articles, get_articles};
use dotenv::dotenv;

fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {} \n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    dotenv().ok();

    let api_key= std::env::var("API_KEY")?;
    let url= std::env::var("API_URL")?;

    let url = format!("{}{}", url, api_key);

    let articles= get_articles(&url)?;

    render_articles(&articles);
    Ok(())
}
