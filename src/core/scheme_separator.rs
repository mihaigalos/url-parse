#[derive(Debug, PartialEq, Eq)]
pub enum SchemaSeparator {
    Colon,
    ColonSlashSlash,
}

impl From<SchemaSeparator> for usize {
    fn from(v: SchemaSeparator) -> usize {
        match v {
            SchemaSeparator::Colon => 1,
            SchemaSeparator::ColonSlashSlash => 3,
        }
    }
}

impl From<SchemaSeparator> for String {
    fn from(v: SchemaSeparator) -> String {
        match v {
            SchemaSeparator::Colon => ":".to_string(),
            SchemaSeparator::ColonSlashSlash => "://".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_separator_to_usize_works_when_colon_typical() {
        let expected = 1;
        let input = SchemaSeparator::Colon;

        let actual: usize = input.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scheme_separator_to_usize_works_when_colon_slash_slash_typical() {
        let expected = 3;
        let input = SchemaSeparator::ColonSlashSlash;

        let actual: usize = input.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scheme_separator_to_string_works_when_colon_typical() {
        let expected = ":".to_string();
        let input = SchemaSeparator::Colon;

        let actual: String = input.into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scheme_separator_to_string_works_when_colon_slash_slash_typical() {
        let expected = "://".to_string();
        let input = SchemaSeparator::ColonSlashSlash;

        let actual: String = input.into();

        assert_eq!(actual, expected);
    }
}