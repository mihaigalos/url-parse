#[derive(Debug, PartialEq, Eq)]
pub enum SchemaSeparator {
    Unknown = 0,
    Colon,
    ColonSlashSlash,
}

impl From<SchemaSeparator> for usize {
    fn from(v: SchemaSeparator) -> usize {
        match v {
            SchemaSeparator::Colon => 1,
            SchemaSeparator::ColonSlashSlash => 3,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_separator_works_when_colon_typical() {
        let expected = 1;
        let input = SchemaSeparator::Colon;

        let actual: usize = input.into();

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_schema_separator_works_when_colon_slash_slash_typical() {
        let expected = 3;
        let input = SchemaSeparator::ColonSlashSlash;

        let actual: usize = input.into();

        assert_eq!(actual, expected);
    }
}
