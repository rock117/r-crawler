use std::collections::HashSet;

mod util;
mod html;
mod crawl;
use crawl::Crawler;
//use reqwest as http;
//use scraper::Html;
//use scraper::Selector;
//use util::file;
//use util::url_parser;
fn main()  {
    let mut crawl = Crawler::new("https://www.163.com/");
    crawl.start();
    println!("hello crawler");
}

















