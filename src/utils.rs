use crate::url::Parser;

pub struct Utils;

impl Utils {
    pub fn substring_after_scheme<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let scheme = parser.mixout_scheme(input);
        let double_slash_length = 2;
        let rest = match scheme.clone() {
            Some(v) => input.get(v.len() + double_slash_length + 1..).unwrap(),
            None => input,
        };
        return rest;
    }

    pub fn substring_after_login<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(&parser, input);
        match input.find("@") {
            Some(pos) => &input[pos + 1..],
            None => input,
        }
    }

    pub fn substring_after_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(&parser, input);
        let port = parser.mixout_port(input);

        if input.find(":").is_some() {
            let (pos_port, len_port_string) = match port {
                Some(v) => (input.find(&v.to_string()).unwrap(), v.to_string().len() + 1),
                None => (0, 0),
            };

            return input.get(pos_port + len_port_string..).unwrap();
        }
        return input;
    }

    pub fn substring_before_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let port = parser.mixout_port(input);

        let pos_port = match port {
            Some(v) => input.find(&v.to_string()).unwrap() - 1,
            None => input.len(),
        };

        return input.get(..pos_port).unwrap();
    }
}

#[test]
fn test_substring_after_scheme_works_when_typical() {
    let input =
        "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected =
        "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_scheme(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_port_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let input = Utils::substring_after_scheme(&parser, input);
    let result = Utils::substring_after_port(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_port_works_when_no_scheme() {
    use crate::url::*;
    let input = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let input = Utils::substring_after_scheme(&parser, input);
    let result = Utils::substring_after_port(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_before_port_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "https://www.example.co.uk".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_before_port(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_login_works_when_typical() {
    let input =
        "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_login(&parser, input);
    assert_eq!(result, expected);
}
