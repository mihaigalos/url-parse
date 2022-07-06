use crate::url::Parser;

pub struct Utils;

impl Utils {
    /// Get substring immediately after scheme.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::url::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected =
    ///     "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_scheme(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_scheme<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let scheme = parser.mixout_scheme(input);
        let double_slash_length = 2;
        match scheme.clone() {
            Some(v) => input.get(v.len() + double_slash_length + 1..).unwrap(),
            None => input,
        }
    }

    /// Get substring immediately after login. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::url::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_login(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_login<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(&parser, input);
        match input.find("@") {
            Some(pos) => &input[pos + 1..],
            None => input,
        }
    }

    /// Get substring immediately after port. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::url::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_login(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(&parser, input);
        let port = parser.mixout_port(input);

        if input.find(":").is_some() {
            let (pos_port, len_port_string) = match port {
                Some(v) => (input.find(&v.to_string()).unwrap(), v.to_string().len() + 1),
                None => (0, 0),
            };

            return input.get(pos_port + len_port_string..).unwrap();
        }
        return input;
    }

    /// Get substring immediately before port.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::url::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "https://www.example.co.uk".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_before_port(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_before_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let port = parser.mixout_port(input);

        let pos_port = match port {
            Some(v) => input.find(&v.to_string()).unwrap() - 1,
            None => input.len(),
        };

        return input.get(..pos_port).unwrap();
    }

    /// Get substring before path. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::url::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected =
    ///     "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_scheme(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_from_path_begin<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(&parser, input);
        match input.find("/") {
            Some(pos) => &input[pos..],
            None => input,
        }
    }
}

#[test]
fn test_substring_after_scheme_works_when_typical() {
    let input =
        "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected =
        "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_scheme(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_port_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_port(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_port_works_when_no_scheme() {
    use crate::url::*;
    let input = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_port(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_before_port_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "https://www.example.co.uk".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_before_port(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_login_works_when_typical() {
    let input =
        "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_login(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_from_path_begin_works_when_typical() {
    use crate::url::*;
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let expected = "/blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_from_path_begin(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_from_path_begin_works_when_no_port() {
    use crate::url::*;
    let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
    let expected = "/blog/article/search?docid=720&hl=en#dayone".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_from_path_begin(&parser, input);
    assert_eq!(result, expected);
}

#[test]
fn test_substring_after_port_works_when_colon_in_url() {
    use crate::url::*;
    let input = "http://en.wikipedia.org/wiki/Template:Welcome";
    let expected = "en.wikipedia.org/wiki/Template:Welcome".to_string();
    let parser = Parser::new(None);
    let result = Utils::substring_after_port(&parser, input);
    assert_eq!(result, expected);
}
