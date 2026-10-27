# md-frontmatter-splitter-cli

This is a CLI tool that reads a Markdown file, splits the YAML front matter and the body, and outputs them as JSON.

## Usage

Pass a Markdown file as an argument or pipe the content of a Markdown file to the tool.
The output will be a JSON object with two keys: `frontmatter` and `content`.

```sh
# File input
md-frontmatter-splitter-cli <path-to-markdown-file>

# Pipe input
cat <path-to-markdown-file> | md-frontmatter-splitter-cli
```

Example using sample.md:

```
% cat sample.md
  ---
  title: Sample Document
  list:
      - item1
      - item2
  ---

  # This is a sample markdown Document

  hello!

% md-frontmatter-splitter-cli sample.md
  {
    "content": "# This is a sample markdown Document\n\nhello!",
    "frontmatter": {
      "list": [
        "item1",
        "item2"
      ],
      "title": "Sample Document"
    }
  }
```

If the input Markdown file does not contain a front matter, the `frontmatter` key will be `null`.

This tool works well with Nushell like this:

```sh
# in Nushell
cat sample.md | md-frontmatter-splitter-cli | from json | get frontmatter.title
```

## Installation

You can install from crates.io using cargo:

```sh
cargo install md-frontmatter-splitter-cli
```

Or you can build and install from source:

```sh
git clone
cd md-frontmatter-splitter-cli
cargo install --path .
#=> ~/.cargo/bin/md-frontmatter-splitter-cli

# or build only, no installation
cargo build --release
#=> ./target/release/md-frontmatter-splitter-cli
```

## Development

You can build, format, lint, and test the project using the following command:

```sh
cargo build && cargo fmt && cargo clippy && cargo test
```

If you have `just` installed, you can also run the following command to build, format, lint, and test the project:

```sh
just
```
