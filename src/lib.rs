//! A library for parsing URLs.
//!
//!No current other crate with support for i.e. special schemes. The reasoning (i.e. in [rust-url](https://github.com/servo/rust-url/pull/776#issuecomment-1159352270)) is that schemes need to be part of the [whatwg](https://url.spec.whatwg.org/#url-miscellaneous) standard to be supported.
//!
//! `url-parse` provides some missing schemes (`sftp`, `ssh`, `s3`) and enables the user to specify custom schemes before parsing.

pub mod core;
pub mod error;
pub mod url;
pub mod utils;
