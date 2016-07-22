extern crate hyper;

use std::io::Read;
use hyper::client::Client;

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