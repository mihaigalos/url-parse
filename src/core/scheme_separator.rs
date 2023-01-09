use alloc::string::{String, ToString};

#[derive(Debug, PartialEq, Eq)]
pub enum SchemeSeparator {
    Colon,
    ColonSlashSlash,
}

impl From<SchemeSeparator> for usize {
    fn from(v: SchemeSeparator) -> usize {
        match v {
            SchemeSeparator::Colon => 1,
            SchemeSeparator::ColonSlashSlash => 3,
        }
    }
}

impl From<SchemeSeparator> for String {
    fn from(v: SchemeSeparator) -> String {
        match v {
            SchemeSeparator::Colon => ":".to_string(),
            SchemeSeparator::ColonSlashSlash => "://".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_separator_to_usize_works_when_colon_typical() {
        let expected = 1;
        let input = SchemeSeparator::Colon;

        let actual: usize = input.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scheme_separator_to_usize_works_when_colon_slash_slash_typical() {
        let expected = 3;
        let input = SchemeSeparator::ColonSlashSlash;

        let actual: usize = input.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scheme_separator_to_string_works_when_colon_typical() {
        let expected = ":".to_string();
        let input = SchemeSeparator::Colon;

        let actual: String = input.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scheme_separator_to_string_works_when_colon_slash_slash_typical() {
        let expected = "://".to_string();
        let input = SchemeSeparator::ColonSlashSlash;

        let actual: String = input.into();

        assert_eq!(actual, expected);
    }
}
