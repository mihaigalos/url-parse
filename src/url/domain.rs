use crate::url::Parser;
use crate::utils::Utils;
use regex::Regex;

#[derive(Debug)]
pub struct DomainFields {
    pub top_level_domain: Option<String>,
    pub domain: Option<String>,
    pub subdomain: Option<String>,
}

impl PartialEq for DomainFields {
    fn eq(&self, other: &Self) -> bool {
        return self.top_level_domain == other.top_level_domain
            && self.domain == other.domain
            && self.subdomain == other.subdomain;
    }
}
impl Parser {
    pub fn mixout_domain_fields<'a>(&self, input: &'a str) -> DomainFields {
        let input = Utils::substring_after_scheme(self, input);
        let re = Regex::new(r"(.*)\.(.*)\.(.*)").unwrap();
        let caps = re.captures(input);
        if caps.is_some() {
            let caps = caps.unwrap();
            return DomainFields {
                top_level_domain: Some(caps.get(1).unwrap().as_str().to_string()),
                domain: Some(caps.get(2).unwrap().as_str().to_string()),
                subdomain: Some(caps.get(3).unwrap().as_str().to_string()),
            };
        } else {
            let re = Regex::new(r"(.*)\.(.*)").unwrap();
            let caps = re.captures(input).unwrap();
            return DomainFields {
                top_level_domain: None,
                domain: Some(caps.get(1).unwrap().as_str().to_string()),
                subdomain: Some(caps.get(2).unwrap().as_str().to_string()),
            };
        }
    }
}

#[test]
fn test_mixout_domain_fields_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = DomainFields {
        top_level_domain: Some("www".to_string()),
        domain: Some("example".to_string()),
        subdomain: Some("com".to_string()),
    };
    let result = Parser::new(None).mixout_domain_fields(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_domain_fields_works_when_typical_long_subdomain() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = DomainFields {
        top_level_domain: Some("www".to_string()),
        domain: Some("example".to_string()),
        subdomain: Some("co.uk".to_string()),
    };
    let result = Parser::new(None).mixout_domain_fields(input);
    assert_eq!(result, expected);
}
