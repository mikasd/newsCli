use std::error::Error;

use colour::{dark_cyan_ln, dark_magenta_ln};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let res = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&res)?;

    Ok(articles)
}

fn display_articles(fetched_articles: &Articles) {
    for article in &fetched_articles.articles {
        dark_cyan_ln!("> {}", article.title);
        dark_magenta_ln!("== {}\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=GET_YOUR_OWN_API_KEY_LOL";
    let articles = get_articles(url)?;

    display_articles(&articles);

    Ok(())
}
