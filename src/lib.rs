extern crate hyper;
extern crate rand;

use std::io::{Read, Write};
use hyper::client::Client;
use rand::Rng;
use std::process;
use std::fs::File;
use std::cmp;

#[cfg(test)]
mod tests {
    #[test]
    fn get_url_content() {
        assert!(::get_url_content("") == None);
    }

    #[test]
    fn rnd_take_n() {
        let mut v = ["aa", "bb"];
        assert!(::rnd_take_n(&mut v, 1).len() == 1);
        assert!(::rnd_take_n(&mut v, 0).len() == 0);
        assert!(::rnd_take_n(&mut v, 2).len() == 2);
        assert!(::rnd_take_n(&mut v, 3).len() == 2);
    }
}

// Get URL content as optional String
pub fn get_url_content(url: &str) -> Option<String> {
    let mut s = String::new();
    Client::new()
        .get(url)
        .send()
        .map(|mut res| res.read_to_string(&mut s).ok().map(|_| s))
        .unwrap_or(None)
}

// Take N random elements
pub fn rnd_take_n<T: Clone>(v: &mut [T], n: usize) -> Vec<T> {
    rand::thread_rng().shuffle(v);
    let r = &v[0..cmp::min(v.len(), n)].to_vec();
    r.to_vec()
}

// Exit with message
pub fn die(message: &str, code: Option<i32>) -> ! {
    println!("{}", message);
    process::exit(code.unwrap_or(1));
}

// Get file contents as String
pub fn file_get_contents(filename: &str) -> Option<String> {
    let mut s = String::new();
    File::open(filename)
        .map(|mut f| f.read_to_string(&mut s).ok().map(|_| s))
        .unwrap_or(None)
}

// Put file contents as String
pub fn file_put_contents(filename: &str, contents: &str) -> Option<()> {
    File::create(filename)
        .map(|mut f| f.write_all(contents.as_bytes()).ok() )
        .unwrap_or(None)
}