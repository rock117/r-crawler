use crate::util::url_parser;
use reqwest as http;

pub struct HtmlPage{
    pub links: Vec<String>
}

impl  HtmlPage{

    pub fn from_html(html: &str) -> Self{
        HtmlPage{links: url_parser::parse(html)}
    }

    pub fn from_url(url: &str) -> Self{
        Self::from_html(http::get(url).unwrap().text().unwrap().as_str())
    }

}