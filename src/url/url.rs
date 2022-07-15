#[derive(Debug)]
pub struct Url {
    pub scheme: Option<String>,
    pub user_pass: (Option<String>, Option<String>),
    pub subdomain: Option<String>,
    pub domain: Option<String>,
    pub top_level_domain: Option<String>,
    pub port: Option<u32>,
    pub path: Option<Vec<String>>,
    pub query: Option<String>,
    pub anchor: Option<String>,
}

impl Url {
    pub fn host(&self) -> Option<String> {
        return match &self.top_level_domain {
            Some(v) => Some(self.domain.as_ref().unwrap().to_owned() + v),
            None => Some(self.domain.as_ref().unwrap().to_owned()),
        };
    }
}
