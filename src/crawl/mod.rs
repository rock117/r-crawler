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

use crate::util::date_util;
use std::sync::atomic::{AtomicUsize, Ordering};
pub struct Crawler {
    entry: String,
    started: bool,
    crawled_urls: HashSet<String>,
    current_num: Arc<AtomicUsize>,
    pool: ThreadPool,
    chanel_num: usize
}

impl Crawler {

    pub fn new(entry: String) -> Self {
        Crawler { entry, started: false, crawled_urls: HashSet::new(), current_num: Arc::new(AtomicUsize::new(0)), pool: ThreadPool::new(50), chanel_num: 500}
    }

    pub fn start(&mut self) {
        if self.started {
            return;
        }
        let url = self.entry.clone();
        let (tx, rx) = sync_channel(self.chanel_num);

        self.init(url, &tx);
        self.loop2(tx, rx)
    }

    fn init(&mut self, url: String, tx: &SyncSender<(String, Response)>) {
        let t = tx.clone();
        let arc = self.current_num.clone();
        self.pool.execute(move || {
            crawl(arc, url, t);
        });
    }

    fn loop2(&mut self, tx: SyncSender<(String, Response)>, rx: Receiver<(String, Response)>) -> () {
        while let Ok((url, mut resp)) = rx.recv() {
            self.mark_crawled(url.clone());
            let urls = self.parse_page_urls(url.as_ref(), read_content(&mut resp).as_ref());
            for url in urls {
                let tx = tx.clone();
                let arc = self.current_num.clone();
                self.pool.execute(move || {
                    crawl(arc, url, tx);
                });
            }
        }
    }

    fn mark_crawled(&mut self, url: String){
        self.crawled_urls.insert(url);
    }

    fn is_done(&self) -> bool {
        false
    }



    fn has_crawled(&self, url: &str) -> bool {
        self.crawled_urls.contains(url)
    }

    fn parse_page_urls(&mut self, parent_url: &str, html: &str) -> Vec<String> {
        let links: Vec<String> = url_parser::parse(html);
        let links = links.iter()
            .map(|url| get_real_url(url, parent_url))
            .filter(|url| !self.has_crawled(url.as_ref()))
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
    let url_parts: Vec<&str> = url.trim_start_matches("https://").trim_start_matches("http://")
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


fn crawl(curr: Arc<AtomicUsize>, url: String,   tx: SyncSender<(String, Response)>){
    let start = date_util::current_milliseconds();
    let resp = http::get(url.as_ref() as &str);
    let end = date_util::current_milliseconds();
    curr.fetch_add(1,Ordering::SeqCst);
    println!("{:?} crawl cost {} ms, url: {}", curr, (end - start), &url);
    if resp.is_err() {
        return;
    }
    let mut resp = resp.unwrap();
    tx.send((url, resp));
}