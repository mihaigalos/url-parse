use std::collections::HashMap;

/// Get the default port mappings for well-known ports.
/// This is a convenience function to create a Parser object (via `Parser::new()`) and pass it defaults.
pub fn default_port_mappings() -> HashMap<&'static str, (u32, &'static str)> {
    HashMap::from([
        ("ftp", (21, "File Transfer Protocol")),
        ("http", (80, "Hypertext Transfer Protocol")),
        ("https", (443, "Hypertext Transfer Protocol Secure")),
        ("ssh", (22, "SSH File Transfer Protocol")),
        ("scp", (22, "SSH File Transfer Protocol")),
        ("sftp", (22, "SSH File Transfer Protocol")),
        ("s3", (443, "Amazon S3 File Transfer Protocol")),
    ])
}
