use reqwest;
use scraper::{Html, Selector};

fn main() {
    // IMDB Top 100 movies page
    const IMDB_WEBSITE: &str =
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100";

    // send a request and extract page source
    let page_source: String = reqwest::blocking::get(IMDB_WEBSITE)
        .unwrap()
        .text()
        .unwrap();

    // parse page source for querying
    let document: Html = Html::parse_document(&page_source);

    // selector h3 tags with the `ipc-title__text` class
    let title_selector: Selector = Selector::parse("h3.ipc-title__text").unwrap();

    // query document with selector and iterate first 100 element extracting string
    let titles = document
        .select(&title_selector)
        .take(100) // 101th element is not a title (wrong selector)
        .map(|item: scraper::ElementRef<'_>| -> String { item.inner_html() });

    // print first 100 titles excluding last element (wrong)
    titles.for_each(|item| println!("{item}"))
}
