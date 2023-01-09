/*!

A library for parsing URLs.

No current other crate with support for i.e. special schemes. The reasoning (i.e. in [rust-url](https://github.com/servo/rust-url/pull/776#issuecomment-1159352270)) is that schemes need to be part of the [whatwg](https://url.spec.whatwg.org/#url-miscellaneous) standard to be supported.

 `url-parse` provides some missing schemes (`sftp`, `ssh`, `s3`) and enables the user to specify custom schemes before parsing.
 # Example
 ```rust,no_run
 use url_parse::core::Parser;
 use url_parse::url::Url;
 let input = "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
 let result = Parser::new(None).parse(input).unwrap();
 assert_eq!(
     result,
     Url {
         scheme: Some("https".to_string()),
         user_pass: (Some("user".to_string()), Some("pass".to_string())),
         subdomain: Some("www".to_string()),
         domain: Some("example.co".to_string()),
         top_level_domain: Some("uk".to_string()),
         port: Some(443),
         path: Some(vec![
             "blog".to_string(),
             "article".to_string(),
             "search".to_string(),
         ]),
         query: Some("docid=720&hl=en#dayone".to_string()),
         anchor: Some("dayone".to_string()),
     }
 )
 ```

 Passing a Some(PortMap) to Parser::new() can be used to create custom schemes.

 The hashmap is a key,value pair representing the scheme name (key) to a port and description mapping (value).
 # Example
 ```rust,no_run
 use url_parse::core::{Parser, PortMap};
 use url_parse::url::Url;
 let input = "myschema://user:pass@example.co.uk/path/to/file.txt";
 let mut myport_mappings = PortMap::new();
 myport_mappings.insert("myschema", (8888, "My custom schema"));
 let result = Parser::new(Some(myport_mappings)).parse(input).unwrap();
 assert_eq!(
     result,
     Url {
         scheme: Some("myschema".to_string()),
         user_pass: (Some("user".to_string()), Some("pass".to_string())),
         subdomain: Some("www".to_string()),
         domain: Some("example.co".to_string()),
         top_level_domain: Some("uk".to_string()),
         port: Some(8888),
         path: Some(vec![
             "path".to_string(),
             "to".to_string(),
             "file.txt".to_string(),
         ]),
         query: None,
         anchor: None,
     }
 )
 ```
*/

#![no_std]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub(crate) type HashMap<K, V> = hashbrown::HashMap<K, V>;
#[cfg(feature = "std")]
pub(crate) type HashMap<K, V> = alloc::collections::HashMap<K, V>;

pub mod core;
pub mod error;
pub mod url;
pub mod utils;
