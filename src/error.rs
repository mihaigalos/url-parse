#[derive(Debug)]
pub enum ParseError {
    UnknownError,
    Ok,
}

// impl From<ParseError> for ParseError {
//     fn from(cause: ParseError) -> std::io::Error {
//         std::io::Error::new(std::io::ErrorKind::Other, cause.to_string())
//     }
// }
