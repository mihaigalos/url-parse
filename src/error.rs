use core::fmt::Display;

#[derive(Debug)]
pub struct ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ParseError")
    }
}

impl std::error::Error for ParseError {}

#[test]
fn test_err_display() {
    let e = ParseError {};
    let x = format!("{e}");
    assert_eq!(x, "ParseError")
}
