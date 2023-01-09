use crate::core::global::Domain;
use crate::core::Parser;
use crate::utils::Utils;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use safe_regex::{regex, Matcher1, Matcher2, Matcher3, Matcher4};
#[cfg(feature = "std")]
use regex::Regex;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use core::str::from_utf8;

impl Parser {
    /// Extract the domain fields from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// let input = "https://www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = Domain {
    ///     subdomain: Some("www"),
    ///     domain: Some("example"),
    ///     top_level_domain: Some("com"),
    /// };
    /// let result = Parser::new(None).domain(input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn domain<'a>(&self, input: &'a str) -> Domain<'a> {
        let input = Utils::substring_after_login(self, input);
        let input = Utils::substring_before_port(self, input);
        let input = match input.find('/') {
            Some(pos) => &input[..pos],
            None => input,
        };
        return self
            .domain_ipv4(input)
            .or_else(|| self.subdomain_domain_top_level_domain(input))
            .or_else(|| self.subdomain_domain(input))
            .or_else(|| self.domain_alias(input))
            .unwrap_or_else(Domain::empty);
    }

    /// Mixes out the subdomain.domain part (i.e.: google.com -> subdomain(None), domain(google), top_level_domain(com))
    #[cfg(feature = "std")]
    fn subdomain_domain<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let re = Regex::new(r"(.*?)\.(.*)").unwrap();
        let caps = re.captures(input);

        caps.as_ref()?;

        let caps = caps.unwrap();
        return Some(Domain {
            subdomain: None,
            domain: Some(caps.get(1).unwrap().as_str()),
            top_level_domain: Some(caps.get(2).unwrap().as_str()),
        });
    }

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    fn subdomain_domain<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let matcher: Matcher2<_> = regex!(br"(.*?)\.(.*)");
        let caps = matcher.match_slices(input.as_bytes())?;

        return Some(Domain {
            subdomain: None,
            domain: Some(from_utf8(caps.0).unwrap()),
            top_level_domain: Some(from_utf8(caps.1).unwrap()),
        });
    }

    /// Mixes out the subdomain.domain.top_level_domain part (i.e.: www.google.com -> subdomain(www), domain(google), top_level_domain(com))
    #[cfg(feature = "std")]
    fn subdomain_domain_top_level_domain<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let re = Regex::new(r"(.*?)\.(.*)\.(.*)").unwrap();
        let caps = re.captures(input);

        caps.as_ref()?;

        let caps = caps.unwrap();
        return Some(Domain {
            subdomain: Some(caps.get(1).unwrap().as_str()),
            domain: Some(caps.get(2).unwrap().as_str()),
            top_level_domain: Some(caps.get(3).unwrap().as_str()),
        });
    }

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    fn subdomain_domain_top_level_domain<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let matcher: Matcher3<_> = regex!(br"(.*?)\.(.*)\.(.*)");
        let caps = matcher.match_slices(input.as_bytes())?;

        return Some(Domain {
            subdomain: Some(from_utf8(caps.0).unwrap()),
            domain: Some(from_utf8(caps.1).unwrap()),
            top_level_domain: Some(from_utf8(caps.2).unwrap()),
        });
    }

    /// Mixes out the ip v4 into a Domain structure.
    #[cfg(feature = "std")]
    fn domain_ipv4<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let re = Regex::new(r"([0-9]+?)\.([0-9]+?)\.([0-9]+?)\.([0-9]+?)").unwrap();
        let caps = re.captures(input);

        caps.as_ref()?;

        return Some(Domain {
            subdomain: None,
            domain: Some(caps.unwrap().get(0).unwrap().as_str()),
            top_level_domain: None,
        });
    }
    
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    fn domain_ipv4<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let matcher: Matcher4<_> = regex!(br"([0-9]+?)\.([0-9]+?)\.([0-9]+?)\.([0-9]+?)");
        let caps = matcher.match_slices(input.as_bytes())?;

        return Some(Domain {
            subdomain: None,
            domain: Some(from_utf8(caps.0).unwrap()),
            top_level_domain: None,
        });
    }

    /// Mixes out single-word alias (i.e.: "localhost") into a Domain structure.
    #[cfg(feature = "std")]
    fn domain_alias<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let re = Regex::new(r".+").unwrap();
        let caps = re.captures(input);

        caps.as_ref()?;

        return Some(Domain {
            subdomain: None,
            domain: Some(caps.unwrap().get(0).unwrap().as_str()),
            top_level_domain: None,
        });
    }

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    fn domain_alias<'a>(&self, input: &'a str) -> Option<Domain<'a>> {
        let matcher: Matcher1<_> = regex!(br"(.+)");
        let caps = matcher.match_slices(input.as_bytes())?;

        return Some(Domain {
            subdomain: None,
            domain: Some(from_utf8(caps.0).unwrap()),
            top_level_domain: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_ipv4_when_typical() {
        let input = "https://1.2.3.4:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = Domain {
            subdomain: None,
            domain: Some("1.2.3.4"),
            top_level_domain: None,
        };
        let result = Parser::new(None).domain_ipv4(input).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_domain_works_when_typical() {
        let input = "https://www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = Domain {
            subdomain: Some("www"),
            domain: Some("example"),
            top_level_domain: Some("com"),
        };
        let result = Parser::new(None).domain(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_domain_works_when_no_subdomain() {
        let input = "https://example.com:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = Domain {
            subdomain: None,
            domain: Some("example"),
            top_level_domain: Some("com"),
        };
        let result = Parser::new(None).domain(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_domain_works_when_typical_long_subdomain() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = Domain {
            subdomain: Some("www"),
            domain: Some("example.co"),
            top_level_domain: Some("uk"),
        };
        let result = Parser::new(None).domain(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_domain_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let expected = Domain {
            subdomain: Some("www"),
            domain: Some("example.co"),
            top_level_domain: Some("uk"),
        };
        let result = Parser::new(None).domain(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subdomain_domain_fails_when_garbage() {
        let input = "foobar";
        let expected = None;
        let result = Parser::new(None).subdomain_domain(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_domain_ipv4_fails_when_garbage() {
        let input = "foobar";
        let expected = None;
        let result = Parser::new(None).subdomain_domain(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_works_when_localhost() {
        let domain = Parser::new(None).domain("ssh://user@localhost:2223/file");
        let result = domain.domain.unwrap();
        assert_eq!(result, "localhost");
    }

    #[test]
    fn test_parse_works_when_empty() {
        let domain = Parser::new(None).domain("");
        let result = domain.domain;
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_works_when_localhost_ip() {
        let domain = Parser::new(None)
            .domain("ftp://127.0.0.1:21/subfolder/test_ftp_put_works_when_subfolder");
        let expected = "127.0.0.1";
        let result = domain.domain.unwrap();
        assert_eq!(result, expected);
    }
}
