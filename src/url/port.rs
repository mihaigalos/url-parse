use regex::Regex;

use crate::url::Parser;

impl Parser {
    pub fn mixout_port<'a>(
        &self,
        input: &'a str,
        scheme: Option<String>,
    ) -> (Option<u32>, &'a str, &'a str) {
        let position_colon = input.find(":");
        if position_colon.is_some() {
            let before = &input[..position_colon.unwrap()];
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
            return (port, before, after);
        }

        return match scheme {
            Some(v) => {
                let (port, _) = self.default_port_mappings[&v.as_ref()];
                (Some(port), "", "")
            }
            None => (None, "", ""),
        };
    }
}

#[test]
fn test_mixout_port_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let (scheme, rest) = Parser::new(None).mixout_scheme(input);
    let (port, _, _) = Parser::new(None).mixout_port(rest, scheme.clone());
    assert_eq!(port.unwrap(), 443);
}

#[test]
fn test_mixout_port_works_when_no_path() {
    let input = "https://www.example.co.uk:443";
    let (scheme, rest) = Parser::new(None).mixout_scheme(input);
    let (port, _, _) = Parser::new(None).mixout_port(rest, scheme.clone());
    assert_eq!(port.unwrap(), 443);
}
#[test]
fn test_mixout_port_default_works_when_http() {
    use crate::url::Parser;
    let input = "https://www.example.co.uk";
    let (scheme, rest) = Parser::new(None).mixout_scheme(input);
    let (port, _, _) = Parser::new(None).mixout_port(rest, scheme.clone());
    assert!(port.is_some());
    assert_eq!(port.unwrap(), 443);
}
