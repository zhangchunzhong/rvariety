extern crate reqwest;

use select::document::Document;
use select::predicate::{Attr, Name};

use super::Index;

struct Wallhaven {
    url: &'static str,
    pattern: &'static str,
    max_page: u32,
}

impl Index for Wallhaven {
    fn new(url: &'static str, pattern: &'static str, max_page: u32) -> Self {
        Wallhaven {
            url,      //"https://wallhaven.cc/"
            pattern,  //"https://wallhaven.cc/search?q=id%3A{}&page={}"
            max_page, //2
        }
    }
    fn make_url(&self, tag: &'static str, page: u32) -> String {
        let str = String::from(self.pattern);
        let m = str.replacen("{}", tag, 1);
        let n = m.replacen("{}", &page.to_string(), 1);
        n
    }
    fn make_filename(&self, _url: &'static str, _tag: &'static str) -> String {
        "".to_string()
    }
    fn parse_urls(&self, tag: &'static str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut rv = Vec::new();
        let mut page: u32 = 1;
        loop {
            let mut uv = Vec::new();
            if page > self.max_page {
                break;
            }
            let url = self.make_url(tag, page);
            let res = reqwest::blocking::get(&url)?;
            if res.status().is_success() == false {
                break;
            }
            println!("page={}, url={}", page, url);
            Document::from_read(res)
                .unwrap()
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x| {
                    if x.find("/w/").is_some() {
                        uv.push(format!("{}", x.to_string()));
                    }
                });
            for u in uv {
                let res = reqwest::blocking::get(&u)?;
                if res.status().is_success() {
                    let document = Document::from_read(res).unwrap();
                    for node in document.find(Name("img")) {
                        if node.attr("id").is_some() {
                            if node.attr("data-cfsrc").is_some() {
                                rv.push(node.attr("data-cfsrc").unwrap().to_string());
                            }
                        }

                    }

                }
            }
            page += 1;
        }
        return Ok(rv);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_make_url() {
        let r = Wallhaven::new(
            "https://wallhaven.cc",
            "https://wallhaven.cc/search?q=id%3A{}&page={}",
            2,
        );
        let u = r.make_url("449", 1);
        assert_eq!(u, "https://wallhaven.cc/search?q=id%3A449&page=1");
    }

    #[test]
    fn test_parse_url() {
        let r = Wallhaven::new(
            "https://wallhaven.cc",
            "https://wallhaven.cc/search?q=id%3A{}&page={}",
            2,
        );
        let m = r.parse_urls("449");
        println!("{:?}", m);
    }
}
