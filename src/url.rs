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
    pub fn host_str(&self) -> Option<String> {
        match &self.top_level_domain {
            Some(v) => Some(self.domain.as_ref().unwrap().to_owned() + "." + v),
            None => Some(self.domain.as_ref().unwrap().to_owned()),
        }
    }

    pub fn port_or_known_default(&self) -> Option<u32> {
        self.port
    }

    pub fn username(&self) -> Option<String> {
        return match &self.user_pass {
            (Some(user), Some(_)) => Some(user.to_owned()),
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
        };
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
        return match &self.user_pass {
            (Some(_), Some(pass)) => Some(pass.to_owned()),
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
        };
    }

    pub fn empty() -> Url {
        Url {
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

impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        return self.scheme == other.scheme
            && self.user_pass == other.user_pass
            && self.subdomain == other.subdomain
            && self.domain == other.domain
            && self.top_level_domain == other.top_level_domain
            && self.port == other.port
            && self.path == other.path
            && self.query == other.query
            && self.anchor == other.anchor;
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
    fn test_password_works_when_typical() {
        let mut input = Url::empty();
        input.user_pass = (Some("user".to_string()), Some("pass".to_string()));

        let result = input.password().unwrap();

        assert_eq!(result, "pass".to_owned());
    }
}
