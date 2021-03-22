// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.
use anyhow::Result;

pub static APP_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.72 Safari/537.36";
pub static GET_URL_INTERVAL:u64 = 2;

trait Index {
    fn new(url: &'static str, pattern: &'static str, max_page: u32) -> Self;
    fn make_url(&self, tag: &'static str, page: u32) -> String;
    fn make_filename(&self, url: &'static str, tag: &'static str) -> String;
    fn parse_urls(&self, tag: &'static str) -> Result<Vec<String>>;
}

pub mod alphacoders;
pub mod wallhaven;
pub mod bing;
pub mod nasa;