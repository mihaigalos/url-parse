use core::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
}
 
impl Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ParseError")
    }
}

impl std::error::Error for ParseError {}