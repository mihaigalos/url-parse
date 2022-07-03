mod anchor;
mod defaults;
mod login;
mod path;
mod port;
mod query;
mod scheme;

use crate::error::ParseError;
use crate::url::defaults::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Url {
    scheme: Option<String>,
    login: Option<(String, String)>,
    _subdomain: Option<String>,
    _domain: Option<String>,
    _top_level_domain: Option<String>,
    port: Option<u32>,
    path: Option<Vec<String>>,
    query: Option<String>,
    anchor: Option<String>,
}

pub struct Parser {
    default_port_mappings: HashMap<&'static str, (u32, &'static str)>,
}

impl Parser {
    pub fn new(port_mappings: Option<HashMap<&'static str, (u32, &'static str)>>) -> Self {
        Parser {
            default_port_mappings: port_mappings.unwrap_or(default_port_mappings()),
        }
    }

    pub fn parse(&self, url: &str) -> Result<Url, ParseError> {
        let scheme = self.mixout_scheme(url);
        let port = self.mixout_port(url);
        let path = self.mixout_path(url);
        let query = self.mixout_query(url);
        let anchor = self.mixout_anchor(url);
        Ok(Url {
            scheme: scheme,
            login: None,
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            port: port,
            path: path,
            query: query,
            anchor: anchor,
        })
    }
}

impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        return self.scheme == other.scheme
            && self.login == other.login
            && self.port == other.port
            && self.path == other.path
            && self.query == other.query
            && self.anchor == other.anchor;
    }
}

#[test]
fn test_parse_works_when_typical() {
    use defaults::DEFAULT_PORT_MAPPINGS;
    for (protocol, _) in DEFAULT_PORT_MAPPINGS.iter() {
        let address = &format!("{}{}", protocol, "foo.bar");
        let url = Parser::new(None).parse(address);
        assert!(url.is_ok());
    }
}

#[test]
fn test_parse_scheme_works_when_typical() {
    use defaults::DEFAULT_PORT_MAPPINGS;
    for (protocol, _) in DEFAULT_PORT_MAPPINGS.iter() {
        let address = &format!("{}://{}", protocol, "foo.bar");
        let url = Parser::new(None).parse(address).unwrap();
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
    use defaults::DEFAULT_PORT_MAPPINGS;
    for (protocol, _) in DEFAULT_PORT_MAPPINGS.iter() {
        let address = &format!("{}{}", protocol, "foo.bar");
        let url = Parser::new(None).parse(address);
        assert!(url.is_ok());
    }
}

#[test]
fn test_parse_works_when_full_url() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).parse(input).unwrap();
    assert_eq!(
        result,
        Url {
            scheme: Some("https".to_string()),
            login: None,
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            port: Some(443),
            path: Some(vec![
                "blog".to_string(),
                "article".to_string(),
                "search".to_string(),
            ]),
            query: Some("docid=720&hl=en#dayone".to_string()),
            anchor: Some("dayone".to_string()),
        }
    );
}

#[test]
fn test_parse_works_when_full_url_with_login() {
    let input =
        "https://user:password@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).parse(input).unwrap();
    assert_eq!(
        result,
        Url {
            scheme: Some("https".to_string()),
            login: None,
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            port: Some(443),
            path: Some(vec![
                "blog".to_string(),
                "article".to_string(),
                "search".to_string(),
            ]),
            query: Some("docid=720&hl=en#dayone".to_string()),
            anchor: Some("dayone".to_string()),
        }
    );
}

#[test]
fn test_parse_works_when_user_login() {
    let input = "scp://user@example.co.uk:22/path/to/file.txt";
    let result = Parser::new(None).parse(input).unwrap();
    assert_eq!(
        result,
        Url {
            scheme: Some("scp".to_string()),
            login: None,
            _subdomain: None,
            _domain: None,
            _top_level_domain: None,
            port: Some(22),
            path: Some(vec![
                "path".to_string(),
                "to".to_string(),
                "file.txt".to_string(),
            ]),
            query: None,
            anchor: None,
        }
    );
}
