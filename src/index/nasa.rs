// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.
//read rss feed from https://www.nasa.gov/rss/dyn/lg_image_of_the_day.rss
extern crate reqwest;

use super::Index;
use anyhow::Result;
use rss::Channel;

struct Nasa {
    //url: &'static str,
    rss_feed: &'static str,
}

impl Index for Nasa {
    fn new(url: &'static str, _pattern: &'static str, _max_page: u32) -> Self {
        Nasa {
            rss_feed: url,
        }
    }
    fn make_url(&self, _tag: &'static str, _page: u32) -> String {
        self.rss_feed.to_string()
    }
    fn make_filename(&self, _url: &'static str, _tag: &'static str) -> String {
        "".to_string()
    }
    fn parse_urls(&self, _tag: &'static str) -> Result<Vec<String>> {
        let mut rv = Vec::new();
        let content = reqwest::blocking::get(self.rss_feed)?.bytes()?;
        let channel = Channel::read_from(&content[..])?;
        for item in channel.items() {
            if item.enclosure.is_some() {
                rv.push(item.enclosure.to_owned().unwrap().url);
            }
        }
        Ok(rv)
    }
}

#[cfg(test)]
mod tests {
    use crate::index::Index;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_parse_urls() {
        let r = Nasa {
            rss_feed: "https://www.nasa.gov/rss/dyn/lg_image_of_the_day.rss",
        };
        let m = r.parse_urls("");
        println!("{:?}", m);
    }
}
