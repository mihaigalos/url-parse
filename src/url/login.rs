use crate::url::Parser;
use regex::Regex;

impl Parser {
    pub fn mixout_login<'a>(&self, input: &'a str) -> Option<(String, String)> {
        let input = self.substring_after_scheme(input);
        let path = self.mixout_path(input);
        let pos_path: usize = match path {
            Some(v) => input.find(&v.join("/")).unwrap(),
            None => input.len(),
        };

        let re = Regex::new(r"(.*)@(.*):.*").unwrap();
        let caps = re.captures(&input[..pos_path]);

        if caps.is_some() {
            let caps = caps.unwrap();
            return if caps.len() > 1 {
                let user_with_pass = caps.get(1).unwrap().as_str().to_string();
                let (user, pass) = match user_with_pass.find(":") {
                    Some(v) => (
                        user_with_pass[..v].to_string(),
                        user_with_pass[v + 1..].to_string(),
                    ),
                    None => (user_with_pass, "".to_string()),
                };
                Some((user, pass))
            } else {
                None
            };
        }
        None
    }
    fn substring_after_scheme<'a>(&self, input: &'a str) -> &'a str {
        let scheme = self.mixout_scheme(input);
        let double_slash_length = 2;
        let rest = match scheme.clone() {
            Some(v) => input.get(v.len() + double_slash_length + 1..).unwrap(),
            None => input,
        };
        return rest;
    }
}

#[test]
fn test_mixout_login_works_when_full_url_with_login() {
    let input =
        "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = ("user".to_string(), "pass".to_string());
    let result = Parser::new(None).mixout_login(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_user_only() {
    let input = "https://user@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = ("user".to_string(), "".to_string());
    let result = Parser::new(None).mixout_login(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_login_works_when_no_login() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_login(input);
    assert!(result.is_none());
}
