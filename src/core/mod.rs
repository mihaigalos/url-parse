mod anchor;
mod defaults;
mod domain;
mod login;
mod path;
mod port;
mod query;
mod scheme;
pub mod scheme_separator;

pub mod global;
use crate::core::defaults::default_port_mappings;
use crate::error::ParseError;
use crate::url::Url;

use std::collections::HashMap;

pub struct Parser {
    port_mappings: HashMap<&'static str, (u32, &'static str)>,
}

impl Parser {
    /// Create a new parser object. Optionally pass in a hash map of default port mappings.
    /// Its fields are then directly accessible.
    ///
    /// # Example
    /// ```rust,no_run
    /// use url_parse::core::Parser;
    /// let parser = Parser::new(None);
    /// ```
    pub fn new(port_mappings: Option<HashMap<&'static str, (u32, &'static str)>>) -> Self {
        Parser {
            port_mappings: port_mappings.unwrap_or_else(default_port_mappings),
        }
    }

    /// Create a new parser object with `Parser::new()`. You can then use `parser.parse(url)` which will return a public `Url` parsed structure back.
    /// Its fields are then directly accessible.
    ///
    /// # Example
    /// ```rust,no_run
    /// use url_parse::core::Parser;
    /// use url_parse::url::Url;
    /// let input = "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let result = Parser::new(None).parse(input).unwrap();
    /// assert_eq!(
    ///     result,
    ///     Url {
    ///         scheme: Some("https".to_string()),
    ///         user_pass: (Some("user".to_string()), Some("pass".to_string())),
    ///         subdomain: Some("www".to_string()),
    ///         domain: Some("example.co".to_string()),
    ///         top_level_domain: Some("uk".to_string()),
    ///         port: Some(443),
    ///         path: Some(vec![
    ///             "blog".to_string(),
    ///             "article".to_string(),
    ///             "search".to_string(),
    ///         ]),
    ///         query: Some("docid=720&hl=en#dayone".to_string()),
    ///         anchor: Some("dayone".to_string()),
    ///     }
    /// )
    /// ```
    pub fn parse(&self, url: &str) -> Result<Url, ParseError> {
        let scheme = self.scheme(url).map(|s| s.0.to_string());
        let user_pass = self.login(url);
        let user_pass = (
            user_pass.0.map(|s| s.to_string()),
            user_pass.1.map(|s| s.to_string()),
        );
        let domain_fields = self.domain(url);
        let port = self.port(url);
        let path = self
            .path(url)
            .map(|x| x.iter().map(|s| s.to_string()).collect());
        let query = self.query(url).map(|s| s.to_string());
        let anchor = self.anchor(url).map(|s| s.to_string());
        Ok(Url {
            scheme,
            user_pass,
            subdomain: domain_fields.subdomain.map(|s| s.to_string()),
            domain: domain_fields.domain.map(|s| s.to_string()),
            top_level_domain: domain_fields.top_level_domain.map(|s| s.to_string()),
            port,
            path,
            query,
            anchor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_works_when_typical() {
        for (protocol, _) in default_port_mappings().iter() {
            let address = &format!("{}{}", protocol, "foo.bar");
            let url = Parser::new(None).parse(address);
            assert!(url.is_ok());
        }
    }

    #[test]
    fn test_parse_scheme_works_when_typical() {
        for (protocol, _) in default_port_mappings().iter() {
            let address = &format!("{}://{}", protocol, "foo.bar");
            let url = Parser::new(None).parse(address).unwrap();
            assert_eq!(&url.scheme.as_ref().unwrap(), protocol);
        }
    }

    #[test]
    fn test_parse_scheme_works_when_no_scheme_in_url() {
        for (protocol, _) in default_port_mappings().iter() {
            let address = &format!("{}{}", protocol, "foo.bar");
            let url = Parser::new(None).parse(address);
            assert!(url.is_ok());
        }
    }

    #[test]
    fn test_parse_works_when_full_url() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).parse(input).unwrap();
        assert_eq!(
            result,
            Url {
                scheme: Some("https".to_string()),
                user_pass: (None, None),
                subdomain: Some("www".to_string()),
                domain: Some("example.co".to_string()),
                top_level_domain: Some("uk".to_string()),
                port: Some(443),
                path: Some(vec![
                    "blog".to_string(),
                    "article".to_string(),
                    "search".to_string(),
                ]),
                query: Some("docid=720&hl=en".to_string()),
                anchor: Some("dayone".to_string()),
            }
        );
    }

    #[test]
    fn test_parse_works_when_full_url_with_login() {
        let input =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).parse(input).unwrap();
        assert_eq!(
            result,
            Url {
                scheme: Some("https".to_string()),
                user_pass: (Some("user".to_string()), Some("pass".to_string())),
                subdomain: Some("www".to_string()),
                domain: Some("example.co".to_string()),
                top_level_domain: Some("uk".to_string()),
                port: Some(443),
                path: Some(vec![
                    "blog".to_string(),
                    "article".to_string(),
                    "search".to_string(),
                ]),
                query: Some("docid=720&hl=en".to_string()),
                anchor: Some("dayone".to_string()),
            }
        );
    }

    #[test]
    fn test_parse_works_when_user_login() {
        let input = "scp://user@example.co.uk:22/path/to/file.txt";
        let result = Parser::new(None).parse(input).unwrap();
        assert_eq!(
            result,
            Url {
                scheme: Some("scp".to_string()),
                user_pass: (Some("user".to_string()), None),
                subdomain: Some("example".to_string()),
                domain: Some("co".to_string()),
                top_level_domain: Some("uk".to_string()),
                port: Some(22),
                path: Some(vec![
                    "path".to_string(),
                    "to".to_string(),
                    "file.txt".to_string(),
                ]),
                query: None,
                anchor: None,
            }
        );
    }

    #[test]
    fn test_parse_works_when_user_login_no_port() {
        let input = "scp://user@example.co.uk/path/to/file.txt";
        let result = Parser::new(None).parse(input).unwrap();
        assert_eq!(
            result,
            Url {
                scheme: Some("scp".to_string()),
                user_pass: (Some("user".to_string()), None),
                subdomain: Some("example".to_string()),
                domain: Some("co".to_string()),
                top_level_domain: Some("uk".to_string()),
                port: Some(22),
                path: Some(vec![
                    "path".to_string(),
                    "to".to_string(),
                    "file.txt".to_string(),
                ]),
                query: None,
                anchor: None,
            }
        );
    }

    #[test]
    fn test_parse_works_when_custom_port_mappings_full_login() {
        let input = "myschema://user:pass@example.co.uk/path/to/file.txt";
        let mut myport_mappings = HashMap::new();
        myport_mappings.insert("myschema", (8888, "My custom schema"));
        let result = Parser::new(Some(myport_mappings)).parse(input).unwrap();
        assert_eq!(
            result,
            Url {
                scheme: Some("myschema".to_string()),
                user_pass: (Some("user".to_string()), Some("pass".to_string())),
                subdomain: Some("example".to_string()),
                domain: Some("co".to_string()),
                top_level_domain: Some("uk".to_string()),
                port: Some(8888),
                path: Some(vec![
                    "path".to_string(),
                    "to".to_string(),
                    "file.txt".to_string(),
                ]),
                query: None,
                anchor: None,
            }
        );
    }

    #[test]
    fn test_parse_works_when_ip() {
        let input = "ftp://192.168.178.242/dir";
        let result = Parser::new(None).parse(input).unwrap();
        assert_eq!(
            result,
            Url {
                scheme: Some("ftp".to_string()),
                user_pass: (None, None),
                subdomain: None,
                domain: Some("192.168.178.242".to_string()),
                top_level_domain: None,
                port: Some(21),
                path: Some(vec![
                    "dir".to_string(),
                ]),
                query: None,
                anchor: None,
            }
        );
    }
}
