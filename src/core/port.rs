use regex::Regex;

use crate::core::Parser;
use crate::utils::Utils;

impl Parser {
    /// Extract the port from the url. If no port is present, it will be deduced from the scheme.
    /// The default scheme provides well-known ports. The user can specify new schemes when constructing the Parser object with `new()`.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let port = Parser::new(None).port(input);
    /// assert_eq!(port.unwrap(), 443);
    /// ```
    pub fn port<'a>(&self, input: &'a str) -> Option<u32> {
        let rest = Utils::substring_after_login(self, input);
        let position_colon = rest.find(":");
        if position_colon.is_some() {
            let _before = &rest[..position_colon.unwrap()];
            let after = &rest[position_colon.unwrap() + 1..];
            let re = Regex::new(r"(\d+).*").unwrap();
            let caps = re.captures(after);
            if caps.is_none() {
                return None;
            }
            let caps = caps.unwrap();

            return Some(caps.get(1).unwrap().as_str().trim().parse::<u32>().unwrap());
        }

        let default_port = match self.scheme(&input.to_string()) {
            Some(v) => {
                let (port, _) = self.default_port_mappings[&v.as_ref()];
                Some(port)
            }
            None => None,
        };
        default_port
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 443);
    }

    #[test]
    fn test_port_works_when_scheme_and_port_specified() {
        let input = "ftp://127.0.0.1:21/test";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 21);
    }

    #[test]
    fn test_port_works_when_no_path() {
        let input = "https://www.example.co.uk:443";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 443);
    }
    #[test]
    fn test_port_default_works_when_https() {
        let input = "https://www.example.co.uk";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 443);
    }

    #[test]
    fn test_port_works_when_default_port_login_and_no_port() {
        let input =
            "https://user:pass@www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).port(input).unwrap();
        assert_eq!(result, 443);
    }
    #[test]
    fn test_port_works_when_login_and_no_port() {
        let input = "user:pass@www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).port(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_port_works_when_login_and_no_port_with_numbers() {
        let input = "user:pass@www.example.co.uk/blog/article/720/test.txt";
        let result = Parser::new(None).port(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_port_works_when_colon_in_url() {
        let input = "http://en.wikipedia.org/wiki/Template:Welcome";
        let result = Parser::new(None).port(input);
        assert!(result.is_none());
    }
}
