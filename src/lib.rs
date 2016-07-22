extern crate hyper;
extern crate rand;

use std::io::Read;
use hyper::client::Client;
use rand::Rng;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(::get_url_content("") == None)
    }
}

pub fn get_url_content(url: &str) -> Option<String> {
    let mut s = String::new();
    Client::new()
        .get(url)
        .send()
        .map(|mut res| res.read_to_string(&mut s).ok().map(|_| s))
        .unwrap_or(None)
}

pub fn rnd_take_n<T: Clone>(v: &mut [T], n: usize) -> Vec<T> {
    rand::thread_rng().shuffle(v);
    let r = &v[0 .. n];
    r.to_vec()
}