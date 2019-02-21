mod util;
extern crate reqwest;
use reqwest as http;
use scraper::Html;
use scraper::Selector;
use util::file;
use util::url_parser;
use log::__private_api_log;

fn main()  {
    let files = file::get_files(r"D:\coding\ide").unwrap();

    println!("{:?}", files);

    let mut resp =  http::get("https://www.douban.com/note/704764170/").unwrap();
    let html = resp.text().unwrap();

    let urls = url_parser::parse(html.as_ref());
    for url in urls.iter() {
        println!("{}", url);
    }

}

fn http_test() -> Result<(), Box<std::error::Error>>{
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