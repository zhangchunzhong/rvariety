// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.
extern crate reqwest;

use select::document::Document;
use select::predicate::Name;


static WEBPAGE: &str = "https://wall.alphacoders.com/tags.php?tid=2315";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let res = reqwest::blocking::get(WEBPAGE)?;
    assert!(res.status().is_success());

    Document::from_read(res)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| if x.find("big.php").is_some() {println!("{}", x)});

    Ok(())
}


