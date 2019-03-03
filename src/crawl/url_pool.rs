use std::collections::VecDeque;
use std::collections::HashSet;

pub struct UrlPool{
    not_crawled: VecDeque<String>,
    crawled: HashSet<String>
}

impl UrlPool{
    pub fn new() -> UrlPool{
        UrlPool{not_crawled: VecDeque::new(), crawled: HashSet::new()}
    }
    pub fn next(&mut self) -> Option<String> {
        match self.not_crawled.pop_back() {
            Some(url) => {
                self.crawled.insert(url.clone());
                Some(url)
            },
            None => None
        }
    }
    pub fn add(&mut self, url: String) {
        if !self.crawled.contains(&url) {
            self.not_crawled.push_front(url)
        }
    }
}
