#[derive(Debug)]
pub struct Domain<'a> {
    pub subdomain: Option<&'a str>,
    pub domain: Option<&'a str>,
    pub top_level_domain: Option<&'a str>,
}

impl<'a> PartialEq for Domain<'a> {
    /// Enables comparison between two domain objects.
    fn eq(&self, other: &Self) -> bool {
        self.subdomain == other.subdomain && self.domain == other.domain
    }
}

impl<'a> Domain<'a> {
    /// Create a new empty domain field with all fields set to none.
    pub fn empty() -> Domain<'a> {
        Domain {
            subdomain: None,
            domain: None,
            top_level_domain: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_empty_when_typical() {
        let expected = Domain {
            subdomain: None,
            domain: None,
            top_level_domain: None,
        };
        let result = Domain::empty();

        assert_eq!(result, expected);
    }
}
