mod util;
mod html;
mod crawl;

use crawl::Crawler;
//use threadpool::ThreadPool;
//use std::time::Duration;
//use std::cell::Cell;

fn main() {
    let mut crawl: Crawler = Crawler::new("https://www.oschina.net/".to_owned());
    crawl.start();
}

struct M {
    name: String
}

impl M {
    fn init(&mut self) {
        println!("init running...");
        let n = self.name.clone();
        let n = n.as_ref();
        self.hello(n);
    }
    fn hello(&mut self, name: &str) {
        println!("hello {}", name);
    }
}


//println!("hello world");
// let mut crawl: Crawler = Crawler::new("https://www.oschina.net/".to_owned());
// crawl.start();











