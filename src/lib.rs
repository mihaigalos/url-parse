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
    _subdomain: Option<String>,
    _domain: Option<String>,
    _top_level_domain: Option<String>,
    _port: Option<u32>,
    _path: Option<String>,
    _query_string_separator: Option<char>,
    _query_string_parameter: Option<String>,
    _fragment: Option<String>,
}

impl Url {
    pub fn parse(url: &str) -> Result<Url, ParseError> {
        let (scheme, _) = Url::mixout_scheme(url);
        Ok(Url {
            scheme: scheme,
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            _port: None,
            _path: None,
            _query_string_separator: None,
            _query_string_parameter: None,
            _fragment: None,
        })
    }

    fn mixout_scheme<'a>(input: &'a str) -> (Option<String>, &'a str) {
        let split: Vec<&str> = input.split("://").collect();

        match split.len() {
            2 => return (Some(split[0].to_string()), split[1]),
            _ => return (None, split[0]),
        };
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
#[test]
fn test_mixout_scheme_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Url::mixout_scheme(input);
    assert_eq!(scheme.unwrap(), "https");
}

#[test]
#[ignore]
fn test_parse_scheme_works_when_full_url() {
    let _input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    for (protocol, _) in PROTOCOLS.iter() {
        let address = &format!("{}{}", protocol, "foo.bar");
        let url = Url::parse(address);
        assert!(url.is_ok());
    }
}
