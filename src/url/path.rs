use crate::url::Parser;

impl Parser {
    pub fn mixout_path<'a>(&self, input: &str) -> Option<Vec<String>> {
        let input = match input.chars().nth(0) {
            Some('/') => &input[1..],
            _ => input,
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
}

#[test]
fn test_mixout_path_works_when_typical() {
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
fn test_mixout_path_works_when_starts_with_slash() {
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
