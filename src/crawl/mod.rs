pub mod url_pool;

use std::collections::HashSet;
use crate::util::url_parser;
use reqwest as http;
use crate::util::url_parser::parse;

pub struct Crawler {
    entry: String,
    started: bool,
    crawled_urls: HashSet<String>,
    current_num: u32
}

impl Crawler {

    pub fn new(entry: String) -> Self {
        Crawler { entry, started: false, crawled_urls: HashSet::new(), current_num: 0}
    }

    pub fn start(&mut self) {
        if self.started {
            return;
        }
        self.started = true;
        let url = self.entry.clone();
        self.crawl(url.as_ref());
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
        if html.is_err() {
            return;
        }
        let html = html.unwrap();
        self.handle_page(url, html.as_str());
    }

    fn has_crawled(&self, url: &str) -> bool {
        self.crawled_urls.contains(url)
    }

    fn handle_page(&mut self, parent_url: &str, html: &str) {
        if self.is_done() {
            eprintln!("Crawl done!");
            return;
        }
        let links: Vec<String> = url_parser::parse(html);
        let links = links.iter()
            .map(|url| get_real_url(url, parent_url))
            .filter(|url| !self.has_crawled(url.as_ref()) && url.contains("oschina.net"))
            .collect::<Vec<String>>();


        for link in links {
            self.crawled_urls.insert(link.to_string());
            self.crawl(link.as_ref());
        }
    }


}
fn get_real_url(url: &str, parent_url: &str) -> String {
    match url {
        u if u.starts_with("//") => format!("{}{}", "https:", u),
        u if u.starts_with("/") => {
            let (protocol, host, _) = parse_url(parent_url).unwrap();
            format!("{}://{}{}", protocol, host, u)
        },
        u => u.to_owned()
    }
}

fn parse_url(url: &str) -> Option<(&str, &str, &str)> {
    if !url.contains("http://") && !url.contains("https://") {
        return None;
    }
    let url_parts: Vec<&str> = url.trim_left_matches("https://").trim_left_matches("http://")
        .split("/").collect();
    let host = url_parts[0];
    let protocol: &str = url.split("://").collect::<Vec<&str>>()[0];
    let path: &str = match url.trim_left_matches(format!("{}://{}", protocol, host).as_ref() as &str) {
        "" => "/",
        p => p
    };
    Some((protocol, host, path))
}
