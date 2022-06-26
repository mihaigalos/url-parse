use regex::Regex;

use crate::url::Url;

impl Url {
    pub fn mixout_port<'a>(input: &'a str) -> (Option<String>, &'a str, &'a str) {
        let position_colon = input.find(":");
        if position_colon.is_some() {
            let before = &input[..position_colon.unwrap()];
            let after = &input[position_colon.unwrap() + 1..];
            let re = Regex::new(r"(\d+).*").unwrap();
            let caps = re.captures(after).unwrap();
            let port = if caps.len() > 1 {
                Some(caps.get(1).unwrap().as_str().to_string())
            } else if caps.len() == 1 {
                let re = Regex::new(r"^(\d+)$").unwrap();
                let caps = re.captures(after).unwrap();
                if caps.len() == 1 {
                    Some(caps.get(0).unwrap().as_str().to_string());
                }
                None
            } else {
                None
            };
            return (port, before, after);
        } else {
        }
        (None, "", "")
    }
}

#[test]
fn test_mixout_port_works_when_typical() {
    let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    let (_, rest) = Url::mixout_scheme(input);
    let (port, _, _) = Url::mixout_port(rest);
    assert_eq!(port.unwrap(), "443");
}

#[test]
fn test_mixout_port_works_when_no_path() {
    let input = "https://www.example.co.uk:443";
    let (_, rest) = Url::mixout_scheme(input);
    let (port, _, _) = Url::mixout_port(rest);
    assert_eq!(port.unwrap(), "443");
}
#[test]
fn test_mixout_port_works_when_no_port() {
    use crate::url::*;
    let input = "https://www.example.co.uk";
    let (_, rest) = Url::mixout_scheme(input);
    let (port, _, _) = Url::mixout_port(rest);
    assert!(port.is_none());
}
