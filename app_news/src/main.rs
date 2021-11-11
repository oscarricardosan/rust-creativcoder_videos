mod theme;

use std::error::Error;
use newsapi::{Articles, get_articles};
use dotenv::dotenv;

fn render_articles(articles: &Articles) {
    let theme= theme::default();
    theme.print_text("# Top headlines\n ");

    for article in &articles.articles {
        theme.print_text(&format!("`{}`", article.title));
        theme.print_text(&format!("> *{}* \n", article.url));
        theme.print_text("---");
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
