use std::error::Error;

struct Articles {
    articles: Vec<Article>
}

struct Article {
    title: String,
    url: String
}

fn get_articles(url: &str) -> Result<Article, Box<dyn Error>> {
    let res = ureq::get(url).call()?.into_string()?;

    dbg!(res);

    todo!()
}

fn main() {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=be85b8f066ce4f16a754cb1cc91f7a60";
    let articles = get_articles(url);
    println!("Hello, world!");
}