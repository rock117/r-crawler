use scraper::Html;
use scraper::Selector;

pub fn parse(html: &str) -> Vec<String>{
    let doc = Html::parse_document(html);
    let mut vec = Vec::new();
    vec.extend(fetch(&doc, "a", "src"));
    vec.extend(fetch(&doc, "script", "src"));
    vec.extend(fetch(&doc, "link", "src"));
    vec.extend(fetch(&doc, "img", "src"));

    vec
}

fn fetch(doc:&Html, tag_name: &str, attr_name: &str) -> Vec<String>{
    let se = Selector::parse(tag_name).unwrap();
    let nodes = doc.select(&se);
    let mut vec:Vec<String> = Vec::new();
    for img in nodes {
        let src = img.value().attr(attr_name);
        if src.is_some() {
            vec.push(String::from(src.unwrap()));
        }
    }
    vec
}