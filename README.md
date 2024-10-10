# Minigrep

Minigrep is a command-line tool for searching text patterns in files, inspired by the classic grep utility. It's written in Rust and offers efficient file processing, case-sensitive and case-insensitive search options, and colorized output.

## Features

- Search for text patterns in files
- Case-sensitive and case-insensitive search options
- Colorized output highlighting the matched text
- Efficient line-by-line file processing, suitable for files of any size
- Command-line interface using clap for easy usage

## Installation

Ensure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/minigrep.git
cd minigrep
cargo build --release
```

The compiled binary will be available in `target/release/minigrep`.

## Usage

Use the following command syntax to run minigrep:

```bash
minigrep --query <SEARCH_TERM> --file-path <FILE_PATH> [--ignore-case]
```

Options:
- `--query` or `-q`: The text pattern to search for (required)
- `--file-path` or `-f`: The path to the file to search in (required)
- `--ignore-case` or `-i`: Perform a case-insensitive search (optional)

Example:
```bash
minigrep --query "Rust" --file-path sample.txt
```

For case-insensitive search:
```bash
minigrep --query "Rust" --file-path sample.txt --ignore-case
```

## Output

Minigrep will display each line containing the search term, with the matched text highlighted in bold red.

## Performance

Minigrep uses efficient line-by-line file processing, allowing it to handle files of any size without loading the entire file into memory. This makes it suitable for searching through large log files or any text files that exceed available RAM.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).
