mod content;
mod frontmatter;

use anyhow::Context;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the file to read (if omitted, reads from standard input)
    file: Option<PathBuf>,
}

fn collect_lines(reader: impl BufRead) -> anyhow::Result<Vec<String>> {
    Ok(reader.lines().collect::<Result<_, _>>()?)
}

fn read_lines(path_opt: Option<&Path>) -> anyhow::Result<Vec<String>> {
    match path_opt {
        Some(path) => {
            let file = File::open(path)
                .with_context(|| format!("Failed to open file: {}", path.display()))?;
            collect_lines(BufReader::new(file))
        }
        None => collect_lines(stdin().lock()),
    }
}

fn read_md(path_opt: Option<PathBuf>) -> anyhow::Result<(yaml_serde::Value, String)> {
    let lines = read_lines(path_opt.as_deref())?;
    let (frontmatter, content) = frontmatter::split(&lines);

    let frontmatter_str = frontmatter.join("\n");
    let frontmatter_yaml: yaml_serde::Value = yaml_serde::from_str(&frontmatter_str)?;

    Ok((frontmatter_yaml, content::join(&content)))
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
