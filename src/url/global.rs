#[derive(Debug)]
pub struct DomainFields<'a> {
    pub subdomain: Option<&'a str>,
    pub domain: Option<&'a str>,
}

impl<'a> PartialEq for DomainFields<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.subdomain == other.subdomain && self.domain == other.domain;
    }
}
