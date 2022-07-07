use crate::url::Parser;
use crate::utils::Utils;
use regex::Regex;

impl Parser {
    /// Extract the domain fields from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::url::Parser;
    /// let input = "https://user:pass@www.example.co.uk";
    /// let expected = (Some("user"), Some("pass"));
    /// let result = Parser::new(None).mixout_login(input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn mixout_login<'a>(&self, input: &'a str) -> (Option<&'a str>, Option<&'a str>) {
        let input = Utils::substring_after_scheme(self, input);
        let input = match input.find("/") {
            Some(pos) => &input[..pos],
            None => input,
        };

        let re = Regex::new(r"(.*)@(.*).*").unwrap();
        let caps = re.captures(input);
        if caps.is_none() {
            return (None, None);
        }

        let caps = caps.unwrap();
        let user_with_pass = caps.get(1).unwrap().as_str();
        let (user, pass) = match user_with_pass.find(":") {
            Some(v) => (Some(&user_with_pass[..v]), Some(&user_with_pass[v + 1..])),
            None => (Some(user_with_pass), None),
        };
        (user, pass)
    }
}

mod tests {
    #[test]
    fn test_mixout_login_works_when_full_url_with_login() {
        use crate::url::*;
        let input =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = (Some("user"), Some("pass"));
        let result = Parser::new(None).mixout_login(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixout_login_works_when_full_url_no_port() {
        use crate::url::*;
        let input =
            "https://user:pass@www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let expected = (Some("user"), Some("pass"));
        let result = Parser::new(None).mixout_login(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixout_login_works_when_full_url_no_port_no_path() {
        use crate::url::*;
        let input = "https://user:pass@www.example.co.uk";
        let expected = (Some("user"), Some("pass"));
        let result = Parser::new(None).mixout_login(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixout_login_works_when_user_only() {
        use crate::url::*;
        let input = "https://user@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = (Some("user"), None);
        let result = Parser::new(None).mixout_login(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixout_login_works_when_user_only_no_port_no_path() {
        use crate::url::*;
        let input = "https://user@www.example.co.uk";
        let expected = (Some("user"), None);
        let result = Parser::new(None).mixout_login(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixout_login_works_when_no_login() {
        use crate::url::*;
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = (None, None);
        let result = Parser::new(None).mixout_login(input);
        assert_eq!(result, expected);
    }
}
