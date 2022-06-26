use regex::Regex;
use std::collections::HashMap;

use crate::error::ParseError;

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

#[derive(Debug)]
struct Url {
    scheme: Option<String>,
    _subdomain: Option<String>,
    _domain: Option<String>,
    _top_level_domain: Option<String>,
    port: Option<String>,
    _path: Option<String>,
    _query_string_separator: Option<char>,
    _query_string_parameter: Option<String>,
    _fragment: Option<String>,
}

impl Url {
    pub fn parse(url: &str) -> Result<Url, ParseError> {
        let (scheme, rest) = Url::mixout_scheme(url);
        let (port, _, _) = Url::mixout_port(rest);
        Ok(Url {
            scheme: scheme,
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            port: port,
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

    fn mixout_port<'a>(input: &'a str) -> (Option<String>, &'a str, &'a str) {
        let position_colon = input.find(":");
        if position_colon.is_some() {
            let before = &input[..position_colon.unwrap()];
            let after = &input[position_colon.unwrap() + 1..];
            let re = Regex::new(r"(\d+).*").unwrap();
            let caps = re.captures(after).unwrap();
            let port = if caps.len() > 1 {
                Some(caps.get(1).unwrap().as_str().to_string())
            } else if caps.len() == 1 {
                let re = Regex::new(r"^(\d+)$").unwrap();
                let caps = re.captures(after).unwrap();
                if caps.len() == 1 {
                    Some(caps.get(0).unwrap().as_str().to_string());
                }
                None
            } else {
                None
            };
            return (port, before, after);
        } else {
        }
        (None, "", "")
    }
}

impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        return self.scheme == other.scheme && self.port == other.port;
    }
}

#[test]
fn test_mixout_port_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let (_, rest) = Url::mixout_scheme(input);
    let (port, _, _) = Url::mixout_port(rest);
    assert_eq!(port.unwrap(), "443");
}

#[test]
fn test_mixout_port_works_when_no_path() {
    let input = "https://www.example.co.uk:443";
    let (_, rest) = Url::mixout_scheme(input);
    let (port, _, _) = Url::mixout_port(rest);
    assert_eq!(port.unwrap(), "443");
}

#[test]
fn test_mixout_port_works_when_no_port() {
    let input = "https://www.example.co.uk";
    let (_, rest) = Url::mixout_scheme(input);
    let (port, _, _) = Url::mixout_port(rest);
    assert!(port.is_none());
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
fn test_parse_works_when_full_url() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Url::parse(input).unwrap();
    assert_eq!(
        result,
        Url {
            scheme: Some("https".to_string()),
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            port: Some("443".to_string()),
            _path: None,
            _query_string_separator: None,
            _query_string_parameter: None,
            _fragment: None,
        }
    );
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

#[test]
fn test_mixout_scheme_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Url::mixout_scheme(input);
    assert_eq!(scheme.unwrap(), "https");
}

#[test]
fn test_mixout_scheme_works_when_no_port() {
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Url::mixout_scheme(input);
    assert_eq!(scheme.unwrap(), "https");
}

#[test]
fn test_mixout_scheme_works_when_no_scheme() {
    let input = "www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Url::mixout_scheme(input);
    assert!(scheme.is_none());
}
