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
    let client = Client::new();
    let mut s = String::new();

    match client.get(url).send() {
        Ok(mut res) => {
            match res.read_to_string(&mut s) {
                Ok(_) => {
                    Some(s)
                },
                Err(_) => {
                    None
                }
            }
        },
        Err(_) => {
            None
        }
    }
}