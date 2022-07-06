//! A library for parsing URLs.
//! Other crates currently do not provide support for i.e. special schemes.
//! That's because they aren't listed in the [whatwg](https://url.spec.whatwg.org/#url-miscellaneous) standard.
//! `url-parse` provides some missing schemes (`sftp`, `ssh`, `s3`) and enables the user to specify custom schemes before parsing.

pub mod error;
pub mod url;
pub mod utils;

#[macro_use]
extern crate lazy_static;
