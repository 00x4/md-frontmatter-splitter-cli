use anyhow::Context;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the file to read (if omitted, reads from standard input)
    file: Option<PathBuf>,
}

fn read_lines_from_file(path: &PathBuf) -> anyhow::Result<Vec<String>> {
    let file = File::open(path).context(format!("Failed to open file: {}", path.display()))?;
    let lines: Vec<String> = BufReader::new(file).lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn read_lines_from_stdin() -> anyhow::Result<Vec<String>> {
    let lines: Vec<String> = BufReader::new(stdin()).lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn read_lines(path_opt: Option<PathBuf>) -> anyhow::Result<Vec<String>> {
    match path_opt {
        Some(path) => read_lines_from_file(&path),
        None => read_lines_from_stdin(),
    }
}

fn split_frontmatter(lines: &[String]) -> (Vec<String>, Vec<String>) {
    // Remove all leading empty lines
    let lines = lines
        .iter()
        .skip_while(|line| line.trim().is_empty())
        .cloned()
        .collect::<Vec<String>>();

    // Trim the BOM character from the first line if present
    let first_line = lines
        .first()
        .map(|line| line.trim_start_matches('\u{feff}').trim_end());

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

fn read_md(path_opt: Option<PathBuf>) -> anyhow::Result<(yaml_serde::Value, String)> {
    let lines = read_lines(path_opt)?;
    let (frontmatter, content) = split_frontmatter(&lines);

    let frontmatter_str = frontmatter.join("\n");
    let frontmatter_yaml: yaml_serde::Value = yaml_serde::from_str(&frontmatter_str)?;

    let content_str = content.join("\n").trim().to_string();
    Ok((frontmatter_yaml, content_str))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (frontmatter, content) = read_md(args.file)?;

    let output = serde_json::json!({
        "frontmatter": serde_json::to_value(&frontmatter)?,
        "content": content,
    });
    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(String::from).collect()
    }

    #[test]
    fn splits_frontmatter_and_content() {
        let (frontmatter, content) = split_frontmatter(&lines("---\ntitle: a\ntags: b\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a\ntags: b"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn content_is_empty_after_closing_delimiter() {
        let (frontmatter, content) = split_frontmatter(&lines("---\ntitle: a\n---"));
        assert_eq!(frontmatter, lines("title: a"));
        assert!(content.is_empty());
    }

    #[test]
    fn frontmatter_is_empty_between_consecutive_delimiters() {
        let (frontmatter, content) = split_frontmatter(&lines("---\n---\nbody"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn no_frontmatter_if_first_line_is_not_delimiter() {
        let (frontmatter, content) = split_frontmatter(&lines("body\n---\ntitle: a\n---"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body\n---\ntitle: a\n---"));
    }

    #[test]
    fn no_frontmatter_if_closing_delimiter_is_missing() {
        let (frontmatter, content) = split_frontmatter(&lines("---\ntitle: a\nbody"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("---\ntitle: a\nbody"));
    }

    #[test]
    fn empty_input() {
        let (frontmatter, content) = split_frontmatter(&[]);
        assert!(frontmatter.is_empty());
        assert!(content.is_empty());
    }

    #[test]
    fn blank_lines_only() {
        let (frontmatter, content) = split_frontmatter(&lines("\n  \n\t\n"));
        assert!(frontmatter.is_empty());
        assert!(content.is_empty());
    }

    #[test]
    fn skips_leading_blank_lines_with_frontmatter() {
        let (frontmatter, content) = split_frontmatter(&lines("\n  \n---\ntitle: a\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn skips_leading_blank_lines_without_frontmatter() {
        let (frontmatter, content) = split_frontmatter(&lines("\n  \nbody\nmore"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("body\nmore"));
    }

    #[test]
    fn strips_bom_from_opening_delimiter() {
        let (frontmatter, content) = split_frontmatter(&lines("\u{feff}---\ntitle: a\n---\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn allows_trailing_whitespace_on_delimiters() {
        let (frontmatter, content) = split_frontmatter(&lines("---  \ntitle: a\n---\t\nbody"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body"));
    }

    #[test]
    fn indented_delimiter_does_not_open_frontmatter() {
        let (frontmatter, content) = split_frontmatter(&lines("  ---\ntitle: a\n---\nbody"));
        assert!(frontmatter.is_empty());
        assert_eq!(content, lines("  ---\ntitle: a\n---\nbody"));
    }

    #[test]
    fn delimiters_after_closing_delimiter_belong_to_content() {
        let (frontmatter, content) =
            split_frontmatter(&lines("---\ntitle: a\n---\nbody\n---\nmore\n---"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("body\n---\nmore\n---"));
    }

    #[test]
    fn content_lines_are_preserved() {
        let (frontmatter, content) =
            split_frontmatter(&lines("---\ntitle: a\n---\n\n    indented\n\n\ttabbed  \n"));
        assert_eq!(frontmatter, lines("title: a"));
        assert_eq!(content, lines("\n    indented\n\n\ttabbed  "));
    }
}
