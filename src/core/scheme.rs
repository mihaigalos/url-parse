use crate::core::scheme_separator::SchemaSeparator;
use crate::core::Parser;
impl Parser {
    /// Extract the query from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::scheme_separator::SchemaSeparator;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let scheme = Parser::new(None).scheme(input);
    /// assert_eq!(scheme.unwrap(), ("https",  SchemaSeparator::ColonSlashSlash));
    /// ```
    ///
    /// Schemas can also have a simple colon instead ot the "://" pattern.
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::scheme_separator::SchemaSeparator;
    /// let input = "https:www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let scheme = Parser::new(None).scheme(input);
    /// assert_eq!(scheme.unwrap(), ("https",  SchemaSeparator::Colon));
    /// ```
    pub fn scheme<'a>(&self, input: &'a str) -> Option<(&'a str, SchemaSeparator)> {
        let split: Vec<&str> = input.split("://").collect();

        if split.len() == 2 {
            return Some((split[0], SchemaSeparator::ColonSlashSlash));
        };

        let split: Vec<&str> = input.split(':').collect();
        let scheme = self
            .default_port_mappings
            .iter()
            .map(|(protocol, _)| protocol)
            .find(|&protocol| &split[0] == protocol)?;
        Some((scheme, SchemaSeparator::Colon))
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
        assert_eq!(scheme.unwrap().0, "https");
    }

    #[test]
    fn test_scheme_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let scheme = Parser::new(None).scheme(input);
        assert_eq!(scheme.unwrap().0, "https");
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
        assert_eq!(scheme.unwrap().0, "https");
    }
}
