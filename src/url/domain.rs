use crate::url::Parser;
use crate::utils::Utils;
use regex::Regex;

#[derive(Debug)]
pub struct DomainFields<'a> {
    pub top_level_domain: Option<&'a str>,
    pub domain: Option<&'a str>,
}

impl<'a> PartialEq for DomainFields<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.top_level_domain == other.top_level_domain && self.domain == other.domain;
    }
}

impl Parser {
    pub fn mixout_domain_fields<'a>(&self, input: &'a str) -> DomainFields<'a> {
        let input = Utils::substring_after_login(self, input);
        let input = Utils::substring_before_port(self, input);
        let input = match input.find("/") {
            Some(pos) => &input[..pos],
            None => input,
        };
        let re = Regex::new(r"(.*?)\.(.*)").unwrap();
        let caps = re.captures(input).unwrap();
        return DomainFields {
            top_level_domain: Some(caps.get(1).unwrap().as_str()),
            domain: Some(caps.get(2).unwrap().as_str()),
        };
    }
}

#[test]
fn test_mixout_domain_fields_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = DomainFields {
        top_level_domain: Some("www"),
        domain: Some("example.com"),
    };
    let result = Parser::new(None).mixout_domain_fields(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_domain_fields_works_when_no_top_level_domain() {
    use crate::url::*;
    let input = "https://example.com:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = DomainFields {
        top_level_domain: Some("example"),
        domain: Some("com"),
    };
    let result = Parser::new(None).mixout_domain_fields(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_domain_fields_works_when_typical_long_subdomain() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = DomainFields {
        top_level_domain: Some("www"),
        domain: Some("example.co.uk"),
    };
    let result = Parser::new(None).mixout_domain_fields(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_domain_fields_works_when_no_port() {
    use crate::url::*;
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let expected = DomainFields {
        top_level_domain: Some("www"),
        domain: Some("example.co.uk"),
    };
    let result = Parser::new(None).mixout_domain_fields(input);
    assert_eq!(result, expected);
}
