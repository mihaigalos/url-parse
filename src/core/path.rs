use crate::core::Parser;
use crate::utils::Utils;

use alloc::vec::Vec;

impl Parser {
    /// Extract the path as a vector from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let result = Parser::new(None).path(input).unwrap();
    /// let expected = vec!["blog", "article", "search"];
    /// assert_eq!(result, expected);
    /// ```
    pub fn path<'a>(&self, input: &'a str) -> Option<Vec<&'a str>> {
        let input = Utils::substring_from_path_begin(self, input);
        let input = Utils::substring_after_port(self, input);
        let input = match input.chars().next() {
            Some('/') => &input[1..],
            _ => input,
        };
        let position_questionmark = input.find('?');
        let path_string = match position_questionmark {
            Some(v) => &input[..v],
            None => input,
        };
        return Some(path_string.split('/').collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_works_when_partial_url() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).path(input).unwrap();
        let expected = vec!["blog", "article", "search"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_path_works_when_partial_url_starts_with_slash() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).path(input).unwrap();
        let expected = vec!["blog", "article", "search"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_path_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).path(input).unwrap();
        let expected = vec!["blog", "article", "search"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_path_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).path(input).unwrap();
        let expected = vec!["blog", "article", "search"];
        assert_eq!(result, expected);
    }
}
