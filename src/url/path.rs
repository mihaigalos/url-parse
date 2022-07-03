use crate::url::Parser;

impl Parser {
    pub fn mixout_path<'a>(&self, input: &str) -> Option<Vec<String>> {
        let input = self.substring_after_port(input).unwrap();
        let input = match input.chars().nth(0) {
            Some('/') => &input[1..],
            _ => &input,
        };
        let position_questionmark = input.find("?");
        if position_questionmark.is_some() {
            return Some(
                (input[..position_questionmark.unwrap()])
                    .split("/")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        } else {
            return Some(
                input
                    .split("/")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        };
    }

    fn substring_after_port(&self, input: &str) -> Option<String> {
        let scheme = self.mixout_scheme(input);
        let rest = match scheme.clone() {
            Some(v) => input.get(v.len() + 1..).unwrap(),
            None => input,
        };
        let port = self.mixout_port(rest);
        return match port {
            Some(v) => {
                let pos_port = rest.find(&v.to_string()).unwrap();
                Some(
                    rest.get(pos_port + v.to_string().len() + 1..)
                        .unwrap()
                        .to_string(),
                )
            }
            None => Some(rest.to_string()),
        };
    }
}

#[test]
fn test_mixout_path_works_when_partial_url() {
    use crate::url::*;
    let input = "blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_path(input).unwrap();
    let expected = vec![
        "blog".to_string(),
        "article".to_string(),
        "search".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_path_works_when_partial_url_starts_with_slash() {
    use crate::url::*;
    let input = "/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_path(input).unwrap();
    let expected = vec![
        "blog".to_string(),
        "article".to_string(),
        "search".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_mixout_path_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_path(input).unwrap();
    let expected = vec![
        "blog".to_string(),
        "article".to_string(),
        "search".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_port_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).substring_after_port(input).unwrap();
    let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
    assert_eq!(result, expected);
}
