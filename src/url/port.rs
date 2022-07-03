use regex::Regex;

use crate::url::Parser;

impl Parser {
    pub fn mixout_port<'a>(&self, input: &'a str) -> Option<u32> {
        let scheme = self.mixout_scheme(input);
        let rest = match scheme.clone() {
            Some(v) => input.get(v.len() + 1..).unwrap(),
            None => input,
        };
        let position_colon = rest.find(":");
        if position_colon.is_some() {
            let _before = &input[..position_colon.unwrap()];
            let after = &input[position_colon.unwrap() + 1..];
            let re = Regex::new(r"(\d+).*").unwrap();
            let caps = re.captures(after).unwrap();
            let port = if caps.len() > 1 {
                Some(caps.get(1).unwrap().as_str().trim().parse::<u32>().unwrap())
            } else if caps.len() == 1 {
                let re = Regex::new(r"^(\d+)$").unwrap();
                let caps = re.captures(after).unwrap();
                if caps.len() == 1 {
                    Some(caps.get(0).unwrap().as_str().trim().parse::<u32>().unwrap());
                }
                None
            } else {
                None
            };
            return port;
        }

        return match scheme {
            Some(v) => {
                let (port, _) = self.default_port_mappings[&v.as_ref()];
                Some(port)
            }
            None => None,
        };
    }
}

#[test]
fn test_mixout_port_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let port = Parser::new(None).mixout_port(input);
    assert_eq!(port.unwrap(), 443);
}

#[test]
fn test_mixout_port_works_when_no_path() {
    let input = "https://www.example.co.uk:443";
    let port = Parser::new(None).mixout_port(input);
    assert_eq!(port.unwrap(), 443);
}
#[test]
fn test_mixout_port_default_works_when_https() {
    use crate::url::Parser;
    let input = "https://www.example.co.uk";
    let port = Parser::new(None).mixout_port(input);
    assert!(port.is_some());
    assert_eq!(port.unwrap(), 443);
}
