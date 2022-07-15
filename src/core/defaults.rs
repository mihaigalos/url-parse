use std::collections::HashMap;

/// Get the default port mappings for well-known ports.
/// This is a convenience function to create a Parser object (via `Parser::new()`) and pass it defaults.
pub fn default_port_mappings() -> HashMap<&'static str, (u32, &'static str)> {
    let mut m = HashMap::new();
    m.insert("ftp", (21, "File Transfer Protocol"));
    m.insert("http", (80, "Hypertext Transfer Protocol"));
    m.insert("https", (443, "Hypertext Transfer Protocol Secure"));
    m.insert("ssh", (22, "SSH File Transfer Protocol"));
    m.insert("scp", (22, "SSH File Transfer Protocol"));
    m.insert("s3", (443, "Amazon S3 File Transfer Protocol"));
    m
}
