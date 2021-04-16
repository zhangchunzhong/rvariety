// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.
use super::index::APP_USER_AGENT;
use anyhow::Result;
use std::fs::File;
use std::io::copy;

fn download(url: &'static str, fname: &'static str) -> Result<u64> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_store(true)
        .build()?;
    let mut res = client.get(url).send()?;
    let mut dest = File::create(fname)?;
    let r = copy(&mut res, &mut dest)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_download() {
        let _m = download(
            "https://www.rust-lang.org/logos/rust-logo-512x512.png",
            "test.png",
        );
    }
}
