#[derive(Debug)]
pub struct Url {
    pub scheme: Option<String>,
    pub user_pass: (Option<String>, Option<String>),
    pub subdomain: Option<String>,
    pub domain: Option<String>,
    pub top_level_domain: Option<String>,
    pub port: Option<u32>,
    pub path: Option<Vec<String>>,
    pub query: Option<String>,
    pub anchor: Option<String>,
}

impl Url {
    /// Extract the representation of the host for this URL.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// let input = "https://user:pass@www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "example.com";
    /// let parsed = Parser::new(None).parse(input).unwrap();
    /// let result = parsed.host_str().unwrap();
    /// assert_eq!(result, expected);
    /// ```
    pub fn host_str(&self) -> Option<String> {
        match &self.top_level_domain {
            Some(v) => Some(self.domain.as_ref().unwrap().to_owned() + "." + v),
            None => Some(self.domain.as_ref().unwrap().to_owned()),
        }
    }

    /// Extract the username from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// let input = "https://user:pass@www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = 443;
    /// let parsed = Parser::new(None).parse(input).unwrap();
    /// let result = parsed.port_or_known_default().unwrap();
    /// assert_eq!(result, expected);
    /// ```
    pub fn port_or_known_default(&self) -> Option<u32> {
        self.port
    }

    /// Extract the username from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// let input = "https://user:pass@www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "user";
    /// let parsed = Parser::new(None).parse(input).unwrap();
    /// let result = parsed.username().unwrap();
    /// assert_eq!(result, expected);
    /// ```
    pub fn username(&self) -> Option<String> {
        match &self.user_pass {
            (Some(user), Some(_)) | (Some(user), None) => Some(user.to_owned()),
            (None, None) => None,
            (None, Some(_)) => None,
        }
    }

    /// Extract the password from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// let input = "https://user:pass@www.example.com:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "pass";
    /// let parsed = Parser::new(None).parse(input).unwrap();
    /// let result = parsed.password().unwrap();
    /// assert_eq!(result, expected);
    /// ```
    pub fn password(&self) -> Option<String> {
        match &self.user_pass {
            (Some(_), Some(pass)) => Some(pass.to_owned()),
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
        }
    }

    /// Extract the path segments from the path.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let result = Parser::new(None).path(input).unwrap();
    /// let expected = vec!["blog", "article", "search"];
    /// assert_eq!(result, expected);
    /// ```
    pub fn path_segments(&self) -> Option<Vec<String>> {
        self.path.clone()
    }

    /// Serialize an URL struct to a String.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// use url_parse::core::global::Domain;
    /// use url_parse::url::Url;
    ///
    ///let input = Url {
    ///    scheme: Some("https".to_string()),
    ///    user_pass: (Some("user".to_string()), Some("pass".to_string())),
    ///    subdomain: Some("www".to_string()),
    ///    domain: Some("example.co".to_string()),
    ///    top_level_domain: Some("uk".to_string()),
    ///    port: Some(443),
    ///    path: Some(vec![
    ///        "blog".to_string(),
    ///        "article".to_string(),
    ///        "search".to_string(),
    ///    ]),
    ///    query: Some("docid=720&hl=en".to_string()),
    ///    anchor: Some("dayone".to_string()),
    ///};
    ///let expected =
    ///    "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    ///
    ///let result = input.serialize();
    /// assert_eq!(result, expected);
    /// ```
    pub fn serialize(&self) -> String {
        let mut result: String = "".to_string();
        if self.scheme.is_some() {
            result += self.scheme.as_ref().unwrap();
            result += "://";
        }
        let (user, pass) = &self.user_pass;
        if user.is_some() {
            result += user.as_ref().unwrap();
        }
        if pass.is_some() {
            result += ":";
            result += pass.as_ref().unwrap();
            result += "@";
        }
        if self.subdomain.is_some() {
            result += self.subdomain.as_ref().unwrap();
            result += ".";
        }
        if self.domain.is_some() {
            result += self.domain.as_ref().unwrap();
            result += ".";
        }
        if self.top_level_domain.is_some() {
            result += self.top_level_domain.as_ref().unwrap();
        }
        if self.port.is_some() {
            result += ":";
            result += &self.port.unwrap().to_string();
        }

        if self.path.is_some() {
            for (_, segment) in self.path_segments().unwrap().iter().enumerate() {
                result += "/";
                result += segment;
            }
        }
        if self.query.is_some() {
            result += "?";
            result += self.query.as_ref().unwrap();
        }
        if self.anchor.is_some() {
            result += "#";
            result += self.anchor.as_ref().unwrap();
        }
        result
    }
    /// Create a new empty instance with all fields set to none.
    pub fn empty() -> Self {
        Self {
            scheme: None,
            user_pass: (None, None),
            subdomain: None,
            domain: None,
            top_level_domain: None,
            port: None,
            path: None,
            query: None,
            anchor: None,
        }
    }
}

/// Compare two objects of this type.
impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        self.scheme == other.scheme
            && self.user_pass == other.user_pass
            && self.subdomain == other.subdomain
            && self.domain == other.domain
            && self.top_level_domain == other.top_level_domain
            && self.port == other.port
            && self.path == other.path
            && self.query == other.query
            && self.anchor == other.anchor
    }
}

/// Display the serialization of this URL.
impl std::fmt::Display for Url {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_works_when_typical() {
        let expected = Url {
            scheme: None,
            user_pass: (None, None),
            subdomain: None,
            domain: None,
            top_level_domain: None,
            port: None,
            path: None,
            query: None,
            anchor: None,
        };
        let result = Url::empty();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_host_works_when_typical() {
        let mut input = Url::empty();
        input.subdomain = Some("abc".to_owned());
        input.domain = Some("def".to_owned());
        input.top_level_domain = Some("xyz".to_owned());

        let result = input.host_str().unwrap();

        assert_eq!(result, "def.xyz".to_owned());
    }

    #[test]
    fn test_extract_host_works_when_no_top_level_domain() {
        let mut input = Url::empty();
        input.subdomain = Some("abc".to_owned());
        input.domain = Some("def".to_owned());
        input.top_level_domain = None;

        let result = input.host_str().unwrap();

        assert_eq!(result, "def".to_owned());
    }

    #[test]
    fn test_port_or_known_default_when_typical() {
        let mut input = Url::empty();
        input.port = Some(1234);

        let result = input.port_or_known_default().unwrap();

        assert_eq!(result, 1234);
    }

    #[test]
    fn test_username_works_when_typical() {
        let mut input = Url::empty();
        input.user_pass = (Some("user".to_string()), Some("pass".to_string()));

        let result = input.username().unwrap();

        assert_eq!(result, "user".to_owned());
    }

    #[test]
    fn test_username_works_when_no_password() {
        let mut input = Url::empty();
        input.user_pass = (Some("user".to_string()), None);

        let result = input.username().unwrap();

        assert_eq!(result, "user".to_owned());
    }

    #[test]
    fn test_username_is_none_when_no_credentials() {
        let input = Url::empty();

        let result = input.username();

        assert!(result.is_none());
    }

    #[test]
    fn test_username_is_none_when_no_username_but_impossible_password() {
        let mut input = Url::empty();
        input.user_pass = (None, Some("pass".to_string()));

        let result = input.username();

        assert!(result.is_none());
    }

    #[test]
    fn test_password_works_when_typical() {
        let mut input = Url::empty();
        input.user_pass = (Some("user".to_string()), Some("pass".to_string()));

        let result = input.password().unwrap();

        assert_eq!(result, "pass".to_owned());
    }

    #[test]
    fn test_password_none_when_no_credentials() {
        let mut input = Url::empty();
        input.user_pass = (None, None);

        let result = input.password();

        assert!(result.is_none());
    }

    #[test]
    fn test_password_none_when_no_password() {
        let mut input = Url::empty();
        input.user_pass = (Some("user".to_string()), None);

        let result = input.password();

        assert!(result.is_none());
    }
    #[test]
    fn test_print_url_when_typical() {
        let input = Url::empty();
        println!("{}", input);
    }

    #[test]
    fn test_path_works_when_partial_url() {
        let mut input = Url::empty();
        let expected = vec![
            "blog".to_string(),
            "article".to_string(),
            "search".to_string(),
        ];
        input.path = Some(expected.clone());

        let result = input.path_segments().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_serialize_to_string() {
        let input = Url {
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
        };
        let expected =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";

        let result = input.serialize();

        assert_eq!(result, expected);
    }
}
