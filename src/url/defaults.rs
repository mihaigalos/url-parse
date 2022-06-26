use std::collections::HashMap;

pub fn default_port_mappings() -> HashMap<&'static str, (u32, &'static str)> {
    let mut m = HashMap::new();
    m.insert("ftp", (21, "File Transfer Protocol"));
    m.insert("http", (80, "Hypertext Transfer Protocol"));
    m.insert("https", (443, "Hypertext Transfer Protocol Secure"));
    m.insert("ssh", (22, "SSH File Transfer Protocol"));
    m.insert("s3", (443, "Amazon S3 File Transfer Protocol"));
    m
}

lazy_static! {
    pub static ref DEFAULT_PORT_MAPPINGS: HashMap<&'static str, (u32, &'static str)> =
        default_port_mappings();
}
