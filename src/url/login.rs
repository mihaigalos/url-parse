use crate::url::Parser;
use crate::utils::Utils;
use regex::Regex;

impl Parser {
    pub fn mixout_login<'a>(&self, input: &'a str) -> (Option<String>, Option<String>) {
        let input = Utils::substring_after_scheme(self, input);
        let input = match input.find("/") {
            Some(pos) => &input[..pos],
            None => input,
        };

        let re = Regex::new(r"(.*)@(.*).*").unwrap();
        let caps = re.captures(input);
        if caps.is_some() {
            let caps = caps.unwrap();
            return if caps.len() > 1 {
                let user_with_pass = caps.get(1).unwrap().as_str().to_string();
                let (user, pass) = match user_with_pass.find(":") {
                    Some(v) => (
                        Some(user_with_pass[..v].to_string()),
                        Some(user_with_pass[v + 1..].to_string()),
                    ),
                    None => (Some(user_with_pass), None),
                };
                (user, pass)
            } else {
                (None, None)
            };
        }
        (None, None)
    }
}

#[test]
fn test_mixout_login_works_when_full_url_with_login() {
    let input =
        "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = (Some("user".to_string()), Some("pass".to_string()));
    let result = Parser::new(None).mixout_login(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_full_url_no_port() {
    let input = "https://user:pass@www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let expected = (Some("user".to_string()), Some("pass".to_string()));
    let result = Parser::new(None).mixout_login(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_full_url_no_port_no_path() {
    let input = "https://user:pass@www.example.co.uk";
    let expected = (Some("user".to_string()), Some("pass".to_string()));
    let result = Parser::new(None).mixout_login(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_user_only() {
    let input = "https://user@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = (Some("user".to_string()), None);
    let result = Parser::new(None).mixout_login(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_user_only_no_port_no_path() {
    let input = "https://user@www.example.co.uk";
    let expected = (Some("user".to_string()), None);
    let result = Parser::new(None).mixout_login(input);
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_no_login() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = (None, None);
    let result = Parser::new(None).mixout_login(input);
    assert_eq!(result, expected);
}
