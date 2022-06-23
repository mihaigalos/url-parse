pub mod error;

use crate::error::ParseError;

struct Url {
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
    let url = Url::parse("http://foo.bar");
    assert!(url.is_ok());
}

#[test]
fn test_parse_scheme_works_when_typical() {
    let url = Url::parse("http://foo.bar").unwrap();
    assert!(url.scheme.unwrap() == "http");
}
