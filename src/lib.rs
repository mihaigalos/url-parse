pub mod error;
use std::collections::HashMap;

use crate::error::ParseError;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PROTOCOLS: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("http", 80);
        m.insert("https", 443);
        m.insert("ftp", 21);
        m.insert("ssh", 22);
        m
    };
}

pub struct Url {
    scheme: Option<String>,
    _port: Option<u32>,
}

impl Url {
    pub fn parse(url: &str) -> Result<Url, ParseError> {
        let split: Vec<&str> = url.split("://").collect();
        let scheme = split[0];
        Ok(Url {
            scheme: Some(scheme.to_string()),
            _port: None,
        })
    }
}

#[test]
fn test_parse_works_when_typical() {
    for (protocol, _) in PROTOCOLS.iter() {
        let address = &format!("{}{}", protocol, "foo.bar");
        let url = Url::parse(address);
        assert!(url.is_ok());
    }
}

#[test]
fn test_parse_scheme_works_when_typical() {
    for (protocol, _) in PROTOCOLS.iter() {
        let address = &format!("{}://{}", protocol, "foo.bar");
        let url = Url::parse(address).unwrap();
        assert!(
            &url.scheme.as_ref().unwrap() == protocol,
            "{} != {}",
            &url.scheme.as_ref().unwrap(),
            protocol
        );
    }
}

#[test]
fn test_parse_scheme_works_when_no_scheme_in_url() {
    for (protocol, _) in PROTOCOLS.iter() {
        let address = &format!("{}{}", protocol, "foo.bar");
        let url = Url::parse(address);
        assert!(url.is_ok());
    }
}
