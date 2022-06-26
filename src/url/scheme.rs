use crate::url::Parser;

impl Parser {
    pub fn mixout_scheme<'a>(input: &'a str) -> (Option<String>, &'a str) {
        let split: Vec<&str> = input.split("://").collect();

        match split.len() {
            2 => return (Some(split[0].to_string()), split[1]),
            _ => return (None, split[0]),
        };
    }
}

#[test]
#[ignore]
fn test_parse_scheme_works_when_full_url() {
    use crate::url::*;
    let _input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    for (protocol, _) in DEFAULT_PORT_MAPPINGS.iter() {
        let address = &format!("{}{}", protocol, "foo.bar");
        let url = Parser::parse(address);
        assert!(url.is_ok());
    }
}

#[test]
fn test_mixout_scheme_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Parser::mixout_scheme(input);
    assert_eq!(scheme.unwrap(), "https");
}

#[test]
fn test_mixout_scheme_works_when_no_port() {
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Parser::mixout_scheme(input);
    assert_eq!(scheme.unwrap(), "https");
}

#[test]
fn test_mixout_scheme_works_when_no_scheme() {
    let input = "www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, _) = Parser::mixout_scheme(input);
    assert!(scheme.is_none());
}
