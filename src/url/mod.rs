mod port;
mod scheme;

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
pub struct Url {
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
}

impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        return self.scheme == other.scheme && self.port == other.port;
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
