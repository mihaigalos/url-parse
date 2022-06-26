use crate::url::Url;

impl Url {
    pub fn mixout_anchor<'a>(input: &'a str) -> Option<String> {
        let position_anchor = input.find("#");
        if position_anchor.is_some() {
            let after = &input[position_anchor.unwrap() + 1..];
            return Some(after.to_string());
        }
        None
    }
}

#[test]
fn test_mixout_anchor_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Url::mixout_anchor(input).unwrap();
    assert_eq!(result, "dayone".to_string());
}
