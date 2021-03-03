// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.


use anyhow::Result;
use serde_json;
use super::Index;

struct Bing {
    url: &'static str,
    pattern: &'static str,
    max: u32,
}

impl Index for Bing {
    fn new(url: &'static str, pattern: &'static str, max: u32) -> Self {
        Bing {
            url,     //https://www.bing.com
            pattern, //https://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n={}&mkt=en-US
            max,
        }
    }
    fn make_url(&self, idx: &'static str, page: u32) -> String {
        let str = String::from(self.pattern);
        let m = str.replacen("{}", idx, 1);
        let n = m.replacen("{}", &page.to_string(), 1);
        n
    }
    fn make_filename(&self, url: &'static str, _tag: &'static str) -> String {
        let s = String::from(url);
        let ps = s.find("id=").unwrap()+3+4;
        let pe = s.find("&").unwrap();
        let slice = &s[ps..pe];
        format!("bing_com/{}", slice)
    }
    fn parse_urls(&self, tag: &'static str) -> Result<Vec<String>, anyhow::Error> {
        let mut rv: Vec<String> = Vec::new();
        let url = self.make_url(tag, self.max);
        let res = reqwest::blocking::get(&url)?.text()?;
        let v: serde_json::Value = serde_json::from_str(&res).expect("Unable to parse");
        match v["images"].to_owned() {
            serde_json::Value::Array(va) => {
                for v in va {
                    let base = String::from(self.url);
                    let mut post = v["url"].to_string();
                    post.pop();
                    post.remove(0);
                    rv.push(format!("{}{}", &base, &post));
                }
            }
            _ => {}
        }
        Ok(rv)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_make_url() {
        let r = Bing::new(
            "https://bing.com",
            "https://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n={}&mkt=en-US",
            2,
        );
        let u = r.make_url("", 1);
        assert_eq!(
            u,
            "https://www.bing.com/HPImageArchive.aspx?format=js&idx=1&n=2&mkt=en-US"
        );
    }
    #[test]
    fn test_parse_url() {
        let r = Bing::new(
            "https://bing.com",
            "https://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n={}&mkt=en-US",
            24,
        );
        let u = r.parse_urls("1");
        println!("{:?}", u);
    }

    #[test]
    fn test_make_filename() {
        let r = Bing::new(
            "https://bing.com",
            "https://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n={}&mkt=en-US",
            24,
        );
        let f = r.make_filename("https://bing.com/th?id=OHR.VolcanoLlaima_ZH-CN3436127573_1920x1080.jpg&rf=LaDigue_1920x1080.jpg&pid=hp", "1");
        println!("{:?}", f);
    }
}
