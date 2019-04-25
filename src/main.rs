mod crawl;
mod util;
use crawl::Crawler;

fn main() {
    let mut crawl: Crawler = Crawler::new("https://www.oschina.net/".to_owned());
    crawl.start();
}










