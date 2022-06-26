use std::collections::HashMap;

pub fn default_port_mappings() -> HashMap<&'static str, u32> {
    let mut m = HashMap::new();
    m.insert("http", 80);
    m.insert("https", 443);
    m.insert("ftp", 21);
    m.insert("ssh", 22);
    m
}

lazy_static! {
    pub static ref DEFAULT_PORT_MAPPINGS: HashMap<&'static str, u32> = default_port_mappings();
}
