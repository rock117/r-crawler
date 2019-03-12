use std::collections::HashSet;

mod util;
mod html;
mod crawl;
use crawl::Crawler;

use reqwest as http;
use scraper::Html;
use scraper::Selector;
use util::file;
use util::url_parser;


struct Tmp{
    age:i32
}
impl Drop for Tmp{
    fn drop(&mut self){
        println!("Tmp drop");
    }
}

fn main()  {
//    let mut crawl = Crawler::new("https://www.163.com/");
//    crawl.start();
//    let url = "https://v.qq.com/x/cover/sduh9vi7i5578p9/n0026vvppfw.html";
//    let path = url.replace("https://","").replace("http://","");
//    println!("path is {}", path);
    let a = "abc".trim_end_matches("c");
    let b = "abc".trim_right_matches("c");
    println!("{}:{}", a, b);
}


















