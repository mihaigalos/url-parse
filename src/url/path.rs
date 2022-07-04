use crate::url::Parser;
use crate::utils::Utils;

impl Parser {
    pub fn mixout_path<'a>(&self, input: &str) -> Option<Vec<String>> {
        let input = Utils::substring_from_path_begin(self, input);
        let input = Utils::substring_after_port(self, input);
        let input = match input.chars().nth(0) {
            Some('/') => &input[1..],
            _ => &input,
        };
        let position_questionmark = input.find("?");
        let path_string = match position_questionmark {
            Some(v) => &input[..v],
            None => input,
        };
        return Some(
            path_string
                .split("/")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }
}

#[test]
fn test_mixout_path_works_when_partial_url() {
    use crate::url::*;
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
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
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
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
fn test_mixout_path_works_when_no_port() {
    use crate::url::*;
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_path(input).unwrap();
    let expected = vec![
        "blog".to_string(),
        "article".to_string(),
        "search".to_string(),
    ];
    assert_eq!(result, expected);
}
