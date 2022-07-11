use crate::url::global::DomainFields;
use crate::url::Parser;
use crate::utils::Utils;
use regex::Regex;

impl Parser {
    /// Extract the domain fields from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::url::Parser;
    /// use url_parse::url::global::DomainFields;
    /// let input = "https://www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = DomainFields {
    ///     subdomain: Some("www"),
    ///     domain: Some("example.com"),
    /// };
    /// let result = Parser::new(None).mixout_domain_fields(input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn mixout_domain_fields<'a>(&self, input: &'a str) -> DomainFields<'a> {
        let input = Utils::substring_after_login(self, input);
        let input = Utils::substring_before_port(self, input);
        let input = match input.find("/") {
            Some(pos) => &input[..pos],
            None => input,
        };
        return self
            .mixout_subdomain_domain(input)
            .unwrap_or_else(|| self.mixout_domain_ipv4(input).unwrap());
    }

    fn mixout_subdomain_domain<'a>(&self, input: &'a str) -> Option<DomainFields<'a>> {
        let re = Regex::new(r"(.*?)\.(.*)").unwrap();
        let caps = re.captures(input);

        if caps.is_none() {
            return None;
        }

        let caps = caps.unwrap();
        return Some(DomainFields {
            subdomain: Some(caps.get(1).unwrap().as_str()),
            domain: Some(caps.get(2).unwrap().as_str()),
        });
    }

    fn mixout_domain_ipv4<'a>(&self, input: &'a str) -> Option<DomainFields<'a>> {
        let re = Regex::new(r"([0-9]+?)\.([0-9]+?)\.([0-9]+?)\.([0-9]+?)").unwrap();
        let caps = re.captures(input);
        if caps.is_none() {
            return None;
        }
        return Some(DomainFields {
            subdomain: None,
            domain: Some(caps.unwrap().get(0).unwrap().as_str()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixout_domain_ipv4_when_typical() {
        let input = "https://1.2.3.4:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = DomainFields {
            subdomain: None,
            domain: Some("1.2.3.4"),
        };
        let result = Parser::new(None).mixout_domain_ipv4(input).unwrap();

        assert_eq!(result, expected);
    }
    #[test]
    fn test_mixout_domain_fields_works_when_typical() {
        let input = "https://www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = DomainFields {
            subdomain: Some("www"),
            domain: Some("example.com"),
        };
        let result = Parser::new(None).mixout_domain_fields(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_mixout_domain_fields_works_when_no_subdomain() {
        let input = "https://example.com:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = DomainFields {
            subdomain: Some("example"),
            domain: Some("com"),
        };
        let result = Parser::new(None).mixout_domain_fields(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_mixout_domain_fields_works_when_typical_long_subdomain() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = DomainFields {
            subdomain: Some("www"),
            domain: Some("example.co.uk"),
        };
        let result = Parser::new(None).mixout_domain_fields(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_mixout_domain_fields_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let expected = DomainFields {
            subdomain: Some("www"),
            domain: Some("example.co.uk"),
        };
        let result = Parser::new(None).mixout_domain_fields(input);
        assert_eq!(result, expected);
    }
}
