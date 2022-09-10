use crate::core::Parser;

impl Parser {
    /// Extract the query from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let scheme = Parser::new(None).scheme(input);
    /// assert_eq!(scheme.unwrap(), "https");
    /// ```
    pub fn scheme<'a>(&self, input: &'a str) -> Option<&'a str> {
        let split: Vec<&str> = input.split("://").collect();

        if split.len() == 2 {
            return Some(split[0]);
        };

        let split: Vec<&str> = input.split(':').collect();
        for (protocol, _) in self.default_port_mappings.iter() {
            if &split[0] == protocol {
                return Some(protocol);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scheme_works_when_simple_address() {
        use crate::core::defaults::*;
        for (protocol, _) in default_port_mappings().iter() {
            let address = &format!("{}{}", protocol, "foo.bar");
            let url = Parser::new(None).parse(address);
            assert!(url.is_ok());
        }
    }

    #[test]
    fn test_scheme_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).scheme(input);
        assert_eq!(scheme.unwrap(), "https");
    }

    #[test]
    fn test_scheme_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).scheme(input);
        assert_eq!(scheme.unwrap(), "https");
    }

    #[test]
    fn test_scheme_works_when_no_scheme() {
        let input = "www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).scheme(input);
        assert!(scheme.is_none());
    }

    #[test]
    fn test_scheme_works_when_no_double_slashes() {
        let input = "https:www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).scheme(input);
        assert_eq!(scheme.unwrap(), "https");
    }
}
