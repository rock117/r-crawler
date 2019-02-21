mod util;
extern crate reqwest;
use reqwest as http;
use scraper::Html;
use scraper::Selector;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut resp =  http::get("https://www.douban.com/note/704764170/").unwrap();
    let str = resp.text();
    // println!("result is: \n{}", str?);
    let doc = Html::parse_document(&str?);
    let se = Selector::parse("img").unwrap();
    let nodes = doc.select(&se);
    for img in nodes {
        let src = img.value().attr("src").unwrap();
        println!("{}", src);
    }
    Ok(())
}
