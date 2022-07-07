use crate::url::Parser;

impl Parser {
    /// Extract the query from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::url::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let scheme = Parser::new(None).mixout_scheme(input);
    /// assert_eq!(scheme.unwrap(), "https");
    /// ```
    pub fn mixout_scheme<'a>(&self, input: &'a str) -> Option<&'a str> {
        let split: Vec<&str> = input.split("://").collect();

        match split.len() {
            2 => return Some(split[0]),
            _ => return None,
        };
    }
}

mod tests {
    #[test]
    fn test_parse_scheme_works_when_simple_address() {
        use crate::url::*;
        for (protocol, _) in DEFAULT_PORT_MAPPINGS.iter() {
            let address = &format!("{}{}", protocol, "foo.bar");
            let url = Parser::new(None).parse(address);
            assert!(url.is_ok());
        }
    }

    #[test]
    fn test_mixout_scheme_works_when_typical() {
        use crate::url::*;
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).mixout_scheme(input);
        assert_eq!(scheme.unwrap(), "https");
    }

    #[test]
    fn test_mixout_scheme_works_when_no_port() {
        use crate::url::*;
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).mixout_scheme(input);
        assert_eq!(scheme.unwrap(), "https");
    }

    #[test]
    fn test_mixout_scheme_works_when_no_scheme() {
        use crate::url::*;
        let input = "www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).mixout_scheme(input);
        assert!(scheme.is_none());
    }
}
