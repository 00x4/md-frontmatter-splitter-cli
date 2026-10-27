/// Joins content lines, dropping leading blank lines and trailing whitespace.
/// The indentation of the first non-blank line is preserved.
pub fn join(lines: &[String]) -> String {
    lines
        .iter()
        .map(String::as_str)
        .skip_while(|line| line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(String::from).collect()
    }

    #[test]
    fn preserves_first_line_indent() {
        assert_eq!(join(&lines("    code\nmore")), "    code\nmore");
    }

    #[test]
    fn drops_leading_blank_lines() {
        assert_eq!(join(&lines("\n  \n    code")), "    code");
    }

    #[test]
    fn drops_trailing_whitespace() {
        assert_eq!(join(&lines("body\n\n  \n")), "body");
    }

    #[test]
    fn empty() {
        assert_eq!(join(&[]), "");
    }
}
