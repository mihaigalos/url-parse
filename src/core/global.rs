#[derive(Debug)]
pub struct Domain<'a> {
    pub subdomain: Option<&'a str>,
    pub domain: Option<&'a str>,
    pub top_level_domain: Option<&'a str>,
}

impl<'a> PartialEq for Domain<'a> {
    /// Enables comparison between two domain objects.
    fn eq(&self, other: &Self) -> bool {
        self.subdomain == other.subdomain && self.domain == other.domain && self.top_level_domain == other.top_level_domain
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

    #[test]
    fn test_domain_equals_when_typical() {
        let left = Domain{ subdomain: Some("one"), domain: Some("two"), top_level_domain: Some("three")};
        let right = Domain{ subdomain: Some("one"), domain: Some("two"), top_level_domain: Some("three")};

        assert_eq!(left, right);
    }

    #[test]
    fn test_domain_not_equals_when_diff_subdomain() {
        let left = Domain{ subdomain: Some("one"), domain: Some("two"), top_level_domain: Some("three")};
        let right = Domain{ subdomain: Some("X"), domain: Some("two"), top_level_domain: Some("three")};

        assert_ne!(left, right);
    }

    #[test]
    fn test_domain_not_equals_when_diff_domain() {
        let left = Domain{ subdomain: Some("one"), domain: Some("two"), top_level_domain: Some("three")};
        let right = Domain{ subdomain: Some("one"), domain: Some("X"), top_level_domain: Some("three")};

        assert_ne!(left, right);
    }

    #[test]
    fn test_domain_not_equals_when_diff_top_level_domain() {
        let left = Domain{ subdomain: Some("one"), domain: Some("two"), top_level_domain: Some("three")};
        let right = Domain{ subdomain: Some("one"), domain: Some("two"), top_level_domain: Some("X")};

        assert_ne!(left, right);
    }
}
