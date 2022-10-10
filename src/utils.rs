use crate::core::schema_separator::SchemaSeparator;
use crate::core::Parser;
use std::collections::HashMap;
pub struct Utils;

impl Utils {
    /// Get substring immediately after scheme.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected =
    ///     "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_scheme(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_scheme<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let scheme = parser.scheme(input);
        match scheme {
            Some((v, separator)) => input
                .get(v.len() + <SchemaSeparator as Into<usize>>::into(separator)..)
                .unwrap(),
            None => input,
        }
    }

    /// Get substring immediately after login. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_login(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_login<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(parser, input);
        match input.find('@') {
            Some(pos) => &input[pos + 1..],
            None => input,
        }
    }

    /// Get substring immediately after port. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_login(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(parser, input);
        let port = parser.port(input);

        if input.find(':').is_some() {
            let (pos_port, len_port_string) = match port {
                Some(v) => (input.find(&v.to_string()).unwrap(), v.to_string().len() + 1),
                None => (0, 0),
            };

            let substring_after_port = input.get(pos_port + len_port_string..);
            return match substring_after_port {
                Some(v) => v,
                None => "",
            };
        }
        input
    }

    /// Get substring immediately before port.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "https://www.example.co.uk".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_before_port(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_before_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let port = parser.port(input);

        let pos_port = match port {
            Some(v) => input.find(&v.to_string()).unwrap() - 1,
            None => input.len(),
        };

        return input.get(..pos_port).unwrap();
    }

    /// Get substring before path. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected =
    ///     "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_scheme(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_from_path_begin<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(parser, input);
        match input.find('/') {
            Some(pos) => &input[pos..],
            None => input,
        }
    }

    /// Partially matches a subpath in a path. Useful for i.e. GitHub absolute paths from URL hrefs.
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
    /// let subpath = "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
    /// let expected = "https://github.com/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
    /// let result = Utils::canonicalize(&Parser::new(None), input, subpath);
    /// assert_eq!(result, expected);
    pub fn canonicalize<'a>(parser: &Parser, input: &'a str, subpath: &'a str) -> String {
        let mut result = parser
            .scheme(input)
            .map(|s| s.0.to_string() + &<SchemaSeparator as Into<String>>::into(s.1))
            .unwrap_or_else(|| "".to_string());

        let (similarity, input_splits) = Utils::compute_similarity(parser, input, subpath);
        let key_with_max_value = similarity.iter().max_by_key(|entry| entry.1).unwrap().0;

        result += &input_splits[0..*key_with_max_value].join("/");
        result = result + "/" + subpath;

        result
    }

    fn compute_similarity<'a>(
        parser: &Parser,
        input: &'a str,
        subpath: &'a str,
    ) -> (HashMap<usize, usize>, Vec<&'a str>) {
        let input = Utils::substring_after_scheme(parser, input);
        let input_splits = input.split('/').collect::<Vec<&str>>();
        let subpath_splits = subpath.split('/').collect::<Vec<&str>>();

        let mut similarity: HashMap<usize, usize> = HashMap::new();
        let mut pos_subpath = 0;
        let mut pos_match = 0;
        for (pos_input, input_split) in input_splits.iter().enumerate() {
            if input_split == &subpath_splits[pos_subpath] {
                if pos_subpath == 0 {
                    pos_match = pos_input;
                }
                pos_subpath += 1;
                *similarity.entry(pos_match).or_insert(0) += 1;
            } else {
                pos_subpath = 0;
            }
        }
        (similarity, input_splits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_after_scheme_works_when_typical() {
        let input =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone"
            .to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_scheme(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_scheme_works_when_simple_schema() {
        let input =
            "https:user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone"
            .to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_scheme(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_no_scheme() {
        let input = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_before_port_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "https://www.example.co.uk".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_before_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_login_works_when_typical() {
        let input =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected =
            "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_login(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_from_path_begin_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "/blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_from_path_begin(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_from_path_begin_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let expected = "/blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_from_path_begin(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_colon_in_url() {
        let input = "http://en.wikipedia.org/wiki/Template:Welcome";
        let expected = "en.wikipedia.org/wiki/Template:Welcome".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_nothing_after_port() {
        let input = "http://192.168.0.100:8080";
        let expected = "".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_similarity_hashmap_works_when_typical() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected_pos_begin_match: usize = 1;
        let expected_count_path_matches: usize = 3;

        let parser = Parser::new(None);
        let (hashmap, _) = Utils::compute_similarity(&parser, input, subpath);
        assert_eq!(
            hashmap[&expected_pos_begin_match],
            expected_count_path_matches
        );
    }

    #[test]
    fn test_compute_similarity_input_splits_works_when_typical() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected_input_splits: Vec<&str> = vec![
            "github.com",
            "mihaigalos",
            "aim",
            "releases",
            "tag",
            "1.5.4",
        ];

        let parser = Parser::new(None);
        let (_, input_splits) = Utils::compute_similarity(&parser, input, subpath);
        assert_eq!(input_splits, expected_input_splits);
    }

    #[test]
    fn test_compute_similarity_works_when_multiple_partial_matches() {
        let input = "https://github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected_pos_begin_match: usize = 5;
        let expected_count_path_matches: usize = 3;

        let parser = Parser::new(None);
        let (hashmap, _) = Utils::compute_similarity(&parser, input, subpath);
        assert_eq!(
            hashmap[&expected_pos_begin_match],
            expected_count_path_matches
        );
    }

    #[test]
    fn test_canonicalize_works_when_typical() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https://github.com/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_multiple_partial_matches() {
        let input = "https://github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https://github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_scheme_with_colon() {
        let input = "https:github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https:github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_no_scheme() {
        let input = "github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_empty() {
        let input = "";
        let subpath = "";
        let expected = "/";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }
}
