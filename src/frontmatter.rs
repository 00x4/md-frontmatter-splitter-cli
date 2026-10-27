/// Splits a slice of lines into frontmatter and content.
pub fn split(lines: &[String]) -> (Vec<String>, Vec<String>) {
    // Trim the BOM character from the first line if present,
    // then remove all leading empty lines
    let lines = lines
        .iter()
        .enumerate()
        .map(|(index, line)| match index {
            0 => line.trim_start_matches('\u{feff}'),
            _ => line.as_str(),
        })
        .skip_while(|line| line.trim().is_empty())
        .map(String::from)
        .collect::<Vec<String>>();

    let first_line = lines.first().map(|line| line.trim_end());

    // No frontmatter if the first line is not '---'
    if first_line != Some("---") {
        return (Vec::new(), lines);
    }

    // No frontmatter if the closing '---' is missing
    let Some(closing) = lines
        .iter()
        .skip(1)
        .position(|line| line.trim_end() == "---")
        .map(|index| index + 1)
    else {
        return (Vec::new(), lines);
    };

    let frontmatter = lines[1..closing].to_vec();
    let content = lines[closing + 1..].to_vec();

    (frontmatter, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(String::from).collect()
    }

    #[test]
    fn splits_frontmatter_and_content() {
        let (frontmatter, content) = split(&lines("---\ntitle: a\ntags: b\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a\ntags: b"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn content_is_empty_after_closing_delimiter() {
        let (frontmatter, content) = split(&lines("---\ntitle: a\n---"));
        assert_eq!(frontmatter, lines("title: a"));
        assert!(content.is_empty());
    }

    #[test]
    fn frontmatter_is_empty_between_consecutive_delimiters() {
        let (frontmatter, content) = split(&lines("---\n---\nbody"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn no_frontmatter_if_first_line_is_not_delimiter() {
        let (frontmatter, content) = split(&lines("body\n---\ntitle: a\n---"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body\n---\ntitle: a\n---"));
    }

    #[test]
    fn no_frontmatter_if_closing_delimiter_is_missing() {
        let (frontmatter, content) = split(&lines("---\ntitle: a\nbody"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("---\ntitle: a\nbody"));
    }

    #[test]
    fn empty_input() {
        let (frontmatter, content) = split(&[]);
        assert!(frontmatter.is_empty());
        assert!(content.is_empty());
    }

    #[test]
    fn blank_lines_only() {
        let (frontmatter, content) = split(&lines("\n  \n\t\n"));
        assert!(frontmatter.is_empty());
        assert!(content.is_empty());
    }

    #[test]
    fn skips_leading_blank_lines_with_frontmatter() {
        let (frontmatter, content) = split(&lines("\n  \n---\ntitle: a\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn skips_leading_blank_lines_without_frontmatter() {
        let (frontmatter, content) = split(&lines("\n  \nbody\nmore"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body\nmore"));
    }

    #[test]
    fn strips_bom_from_opening_delimiter() {
        let (frontmatter, content) = split(&lines("\u{feff}---\ntitle: a\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn strips_bom_without_frontmatter() {
        let (frontmatter, content) = split(&lines("\u{feff}body\nmore"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body\nmore"));
    }

    #[test]
    fn strips_bom_on_otherwise_blank_first_line() {
        let (frontmatter, content) = split(&lines("\u{feff}\n---\ntitle: a\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn allows_trailing_whitespace_on_delimiters() {
        let (frontmatter, content) = split(&lines("---  \ntitle: a\n---\t\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn indented_delimiter_does_not_open_frontmatter() {
        let (frontmatter, content) = split(&lines("  ---\ntitle: a\n---\nbody"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("  ---\ntitle: a\n---\nbody"));
    }

    #[test]
    fn delimiters_after_closing_delimiter_belong_to_content() {
        let (frontmatter, content) = split(&lines("---\ntitle: a\n---\nbody\n---\nmore\n---"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body\n---\nmore\n---"));
    }

    #[test]
    fn content_lines_are_preserved() {
        let (frontmatter, content) =
            split(&lines("---\ntitle: a\n---\n\n    indented\n\n\ttabbed  \n"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("\n    indented\n\n\ttabbed  "));
    }
}
