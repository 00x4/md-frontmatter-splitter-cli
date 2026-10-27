mod frontmatter;

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

fn read_md(path_opt: Option<PathBuf>) -> anyhow::Result<(yaml_serde::Value, String)> {
    let lines = read_lines(path_opt)?;
    let (frontmatter, content) = frontmatter::split(&lines);

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
