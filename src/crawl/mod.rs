pub mod url_pool;

use std::collections::HashSet;
use crate::util::url_parser;
use reqwest as http;

pub struct Crawler<'a> {
    entry: &'a str,
    started: bool,
    crawled_urls: HashSet<String>,
    current_num: u32
}

impl<'a> Crawler<'a> {
    pub fn new(entry: &'a str) -> Self {
        Crawler{entry: entry, started: false, crawled_urls: HashSet::new(), current_num: 0}
    }

    pub fn start(&mut self) {
        if self.started {
            return;
        }
        self.started = true;
        self.crawl(self.entry);
    }

    fn is_done(&self) -> bool {
        self.current_num > 1009
    }

    fn crawl(&mut self, url: &str) {
        self.current_num = self.current_num + 1;

        println!("{} Begin to fetch {}", self.current_num, url);
        let resp = http::get(url);
        if resp.is_err() {
            return;
        }
        let html = resp.unwrap().text();
        if(html.is_err()){
            return;
        }
        let html = html.unwrap();
        let html = html.as_str();
        self.handle_page(html);
    }

    fn has_crawled(&self, url: &str) -> bool {
        self.crawled_urls.contains(url)
    }

    fn handle_page(&mut self, html: &str){
        if self.is_done() {
            eprintln!("Crawl done!");
            return;
        }
        let links = url_parser::parse(html);
        let links = links.iter()
            .filter(|&url| !self.has_crawled(url) && url != "#")
            .collect::<Vec<&String>>();

        for link in links.iter() {
            self.crawled_urls.insert(link.to_string());
            self.crawl(link);
        }
    }
}


