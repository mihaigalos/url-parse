use crate::url::Parser;

impl Parser {
    pub fn mixout_path<'a>(&self, input: &str) -> Option<Vec<String>> {
        let input = self.substring_after_port(input);
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

    fn substring_after_port<'a>(&self, input: &'a str) -> &'a str {
        let scheme = self.mixout_scheme(input);
        let rest = match scheme.clone() {
            Some(v) => input.get(v.len() + 1..).unwrap(),
            None => input,
        };
        let port = self.mixout_port(rest);

        let (pos_port, len_port_string) = match port {
            Some(v) => (rest.find(&v.to_string()).unwrap(), v.to_string().len() + 1),
            None => (0, 0),
        };

        return rest.get(pos_port + len_port_string..).unwrap();
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
    let result = Parser::new(None).substring_after_port(input);
    let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
    assert_eq!(result, expected);
}
