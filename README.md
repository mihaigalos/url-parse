# url-parse

[![CI](https://github.com/mihaigalos/url-parse/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/mihaigalos/url-parse/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/mihaigalos/url-parse/branch/main/graph/badge.svg?token=crukaI8Gmf)](https://codecov.io/gh/mihaigalos/url-parse)
[![crates.io](https://img.shields.io/crates/d/url-parse.svg)](https://crates.io/crates/url-parse)

A library for parsing URLs.

## Why?

Currently, [url](https://crates.io/crates/url) does not provide support for i.e. special schemes. That's because they aren't listed in the [whatwg](https://url.spec.whatwg.org/#url-miscellaneous) standard.

`url-parse` provides some missing schemes (`sftp`, `ssh`, `s3`) and enables the user to specify custom schemes before parsing.

## Usage

### Basic

Create a new parser object with `Parser::new()`. You can then use `parser.parse(url)` which will return a public `Url` parsed structure back.

Its fields are then directly accessible:

```rust
let input = "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
let result = Parser::new(None).parse(input).unwrap();
assert_eq!(
    result,
    Url {
        scheme: Some("https".to_string()),
        user_pass: (Some("user".to_string()), Some("pass".to_string())),
        top_level_domain: Some("www".to_string()),
        domain: Some("example.co.uk".to_string()),
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

### Custom schemes

Passing a `Some(HashMap)` to `Parser::new()` can be used to create custom schemes.

The hashmap is a key,value pair representing the scheme name (key) to a port and description mapping (value).

```rust
    let input = "myschema://user:pass@example.co.uk/path/to/file.txt";
    let mut myport_mappings = HashMap::new();
    myport_mappings.insert("myschema", (8888, "My custom schema"));
    let result = Parser::new(Some(myport_mappings)).parse(input).unwrap();
    assert_eq!(
        result,
        Url {
            scheme: Some("myschema".to_string()),
            user_pass: (Some("user".to_string()), Some("pass".to_string())),
            top_level_domain: Some("example".to_string()),
            domain: Some("co.uk".to_string()),
            port: Some(8888),
            path: Some(vec![
                "path".to_string(),
                "to".to_string(),
                "file.txt".to_string(),
            ]),
            query: None,
            anchor: None,
        }
    );
```