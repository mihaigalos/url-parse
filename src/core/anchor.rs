use crate::core::Parser;

impl Parser {
    /// Extract the anchor from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let result = Parser::new(None).anchor(input).unwrap();
    /// assert_eq!(result, "dayone");
    /// ```
    pub fn anchor<'a>(&self, input: &'a str) -> Option<&'a str> {
        let position_anchor = input.find('#');
        if let Some(v) = position_anchor {
            let after = &input[v + 1..];
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
