use crate::url::Parser;

impl Parser {
    /// Extract the anchor from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::url::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let result = Parser::new(None).anchor(input).unwrap();
    /// assert_eq!(result, "dayone");
    /// ```
    pub fn anchor<'a>(&self, input: &'a str) -> Option<&'a str> {
        let position_anchor = input.find("#");
        if position_anchor.is_some() {
            let after = &input[position_anchor.unwrap() + 1..];
            return Some(after);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).anchor(input).unwrap();
        assert_eq!(result, "dayone");
    }
}
