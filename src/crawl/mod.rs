pub mod url_pool;

use std::collections::HashSet;
use crate::util::url_parser;
use reqwest as http;
use crate::util::url_parser::parse;
use std::sync::{Arc, Mutex};
use reqwest::Response;
use threadpool::ThreadPool;

use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::io::Read;

pub struct Crawler {
    entry: String,
    started: bool,
    crawled_urls: HashSet<String>,
    current_num: i32,
    pool: ThreadPool
}

impl Crawler {

    pub fn new(entry: String) -> Self {
        Crawler { entry, started: false, crawled_urls: HashSet::new(), current_num: 0, pool: ThreadPool::new(50)}
    }

    pub fn start(&mut self) {
        if self.started {
            return;
        }
        let url = self.entry.clone();
        let (tx, rx) = sync_channel(10);

        self.init(url, &tx);
        self.loop2(tx, rx)
    }

    fn init(&mut self, url: String, tx: &SyncSender<(String, Response)>) {
        let t = tx.clone();
        let n = self.current_num;
        self.pool.execute(move || {
            crawl(n, url, t);
        });
    }

    fn loop2(&mut self, tx: SyncSender<(String, Response)>, rx: Receiver<(String, Response)>) -> () {
        while let Ok((url, mut resp)) = rx.recv() {
            let urls = self.handle_page(url.as_ref(), read_content(&mut resp).as_ref());
            for url in urls {
                let tx = tx.clone();
                self.current_num = self.current_num + 1;
                let n = self.current_num;
                self.pool.execute(move || {
                    crawl(n, url, tx);
                });
            }
        }
    }

    fn is_done(&self) -> bool {
        self.current_num > 1009
    }



    fn has_crawled(&self, url: &str) -> bool {
        self.crawled_urls.contains(url)
    }

    fn handle_page(&mut self, parent_url: &str, html: &str) -> Vec<String> {
        let links: Vec<String> = url_parser::parse(html);
        let links = links.iter()
            .map(|url| get_real_url(url, parent_url))
            .filter(|url| !self.has_crawled(url.as_ref()) && url.contains("oschina.net"))
            .collect::<Vec<String>>();
        links
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
    let path: &str = match url.trim_start_matches(format!("{}://{}", protocol, host).as_ref() as &str) {
        "" => "/",
        p => p
    };
    Some((protocol, host, path))
}

fn read_content(resp: &mut Response) -> String {
    let mut buf = String::new();
    resp.read_to_string(&mut buf);
    buf
}


fn crawl(curr: i32, url: String,   tx: SyncSender<(String, Response)>){
    println!("Begin to crawl {} {}", curr, &url);
    let resp = http::get(url.as_ref() as &str);
    if resp.is_err() {
        return;
    }
    let mut resp = resp.unwrap();
    tx.send((url, resp));
}