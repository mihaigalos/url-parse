use crate::url::Url;

impl Url {
    pub fn mixout_query<'a>(input: &'a str) -> Option<String> {
        let position_colon = input.find("?");
        if position_colon.is_some() {
            let after = &input[position_colon.unwrap() + 1..];
            return Some(after.to_string());
        }
        None
    }
}

#[test]
fn test_mixout_query_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let result = Url::mixout_query(input).unwrap();
    assert_eq!(result, "docid=720&hl=en#dayone".to_string());
}
