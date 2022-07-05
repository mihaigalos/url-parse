use crate::url::Parser;

impl Parser {
    pub fn mixout_anchor<'a>(&self, input: &'a str) -> Option<&'a str> {
        let position_anchor = input.find("#");
        if position_anchor.is_some() {
            let after = &input[position_anchor.unwrap() + 1..];
            return Some(after);
        }
        None
    }
}

#[test]
fn test_mixout_anchor_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Parser::new(None).mixout_anchor(input).unwrap();
    assert_eq!(result, "dayone");
}
