#[derive(Debug)]
pub struct DomainFields<'a> {
    pub subdomain: Option<&'a str>,
    pub domain: Option<&'a str>,
    pub top_level_domain: Option<&'a str>,
}

impl<'a> PartialEq for DomainFields<'a> {
    /// Enables comparison between two domain objects.
    fn eq(&self, other: &Self) -> bool {
        return self.subdomain == other.subdomain && self.domain == other.domain;
    }
}

impl<'a> DomainFields<'a> {
    /// Create a new empty domain field with all fields set to none.
    pub fn empty() -> DomainFields<'a> {
        DomainFields {
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
        let expected = DomainFields {
            subdomain: None,
            domain: None,
            top_level_domain: None,
        };
        let result = DomainFields::empty();

        assert_eq!(result, expected);
    }
}
