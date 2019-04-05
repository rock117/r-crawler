mod util;
mod html;
mod crawl;

use crawl::Crawler;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use libc;
use std::io::Write;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut crawl: Crawler = Crawler::new("https://www.oschina.net/".to_owned());
    crawl.start();


}
