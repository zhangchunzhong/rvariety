// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.
extern crate reqwest;

use regex::Regex;
use select::document::Document;
use select::predicate::Name;

use super::Index;

struct Alphacoders {
    url: &'static str,
    pattern: &'static str,
    max_page: u32,
}

impl Index for Alphacoders {
    fn new(url: &'static str, pattern: &'static str, max_page: u32) -> Self {
        Alphacoders {
            url,     //"https://wall.alphacoders.com"
            pattern, //"https://wall.alphacoders.com/tags.php?tid={}&page={}"
            max_page,
        }
    }
    fn make_url(&self, tag: &'static str, page: u32) -> String {
        let str = String::from(self.pattern);
        let m = str.replacen("{}", tag, 1);
        let n = m.replacen("{}", &page.to_string(), 1);
        n
    }
    fn make_filename(&self, url: &'static str, tag: &'static str) -> String {
        let re = Regex::new(r"^https://[0-9a-z]+[\.](?P<m>[a-z]+)[\.](?P<y>[a-z]+)/\d+/(?P<f>\d+)[\.]jpg$").unwrap();
        let prefix = re.replace_all(url, "$m");
        let post = re.replace_all(url, "$y").to_string();
        let file = re.replace_all(url, "$f").to_string();
        let mut str = prefix.to_string();
        str.push_str("_");
        str.push_str(&post);
        str.push_str("_tag_");
        str.push_str(tag);
        str.push('/');
        str.push_str(&file);
        str.push_str(".jpg");
        str
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
                    if x.find("big.php").is_some() {
                        uv.push(format!("{}/{}", self.url, x.to_string()));
                    }
                });
            for u in uv {
                let res = reqwest::blocking::get(&u)?;
                assert!(res.status().is_success());
                Document::from_read(res)
                    .unwrap()
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .for_each(|x| {
                        let re = Regex::new(r"^https://[0-9a-z.]*/\d*/\d*.jpg$").unwrap();
                        if re.is_match(x) {
                            println!("    pic={}", x);
                            rv.push(x.to_string());
                        }
                    });
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
        let r = Alphacoders::new(
            "https://wall.alphacoders.com",
            "https://wall.alphacoders.com/tags.php?tid={}&page={}",
            2
        );
        let u = r.make_url("2315", 1);
        assert_eq!(u, "https://wall.alphacoders.com/tags.php?tid=2315&page=1");
    }
    #[test]
    fn test_parse_url() {
        let r = Alphacoders::new(
            "https://wall.alphacoders.com",
            "https://wall.alphacoders.com/tags.php?tid={}&page={}",
            2
        );
        let m = r.parse_urls("2315");
        println!("{:?}", m);
    }
    #[test]
    fn test_make_filename() {
        let r = Alphacoders::new(
            "https://wall.alphacoders.com",
            "https://wall.alphacoders.com/tags.php?tid={}&page={}",
            2
        );
        let m = r.make_filename("https://images5.alphacoders.com/739/739383.jpg", "2315");
        println!("{}", m);
    }
}
