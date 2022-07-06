#[derive(Debug)]
pub struct DomainFields<'a> {
    pub top_level_domain: Option<&'a str>,
    pub domain: Option<&'a str>,
}

impl<'a> PartialEq for DomainFields<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.top_level_domain == other.top_level_domain && self.domain == other.domain;
    }
}
