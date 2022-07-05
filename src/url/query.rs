use crate::url::Parser;

impl Parser {
    pub fn mixout_query<'a>(&self, input: &'a str) -> Option<&'a str> {
        let position_questionmark = input.find("?");
        if position_questionmark.is_some() {
            let after = &input[position_questionmark.unwrap() + 1..];
            return Some(after);
        }
        None
    }
}

#[test]
fn test_mixout_query_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_query(input).unwrap();
    assert_eq!(result, "docid=720&hl=en#dayone".to_string());
}
