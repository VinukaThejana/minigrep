use clap::Parser;
use minigrep::{colorize, Config};
use std::process::{self};

/// Search for a query in a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The query to search for
    #[arg(short, long)]
    query: String,

    /// The file to search in
    #[arg(short, long)]
    file_path: std::path::PathBuf,

    /// Ignore case when searching
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();
    let mut matched = Vec::new();

    let config = Config::new(args.query, args.file_path, args.ignore_case);
    config
        .run(|line| {
            if config.ignore_case {
                if line.to_lowercase().contains(&config.query.to_lowercase()) {
                    matched.push(colorize(&config.query, line, config.ignore_case));
                }
                return Ok(());
            }

            if line.contains(&config.query) {
                matched.push(colorize(&config.query, line, config.ignore_case));
            }

            Ok(())
        })
        .unwrap_or_else(|err| {
            match err.kind() {
                std::io::ErrorKind::NotFound => eprintln!("file not found"),
                std::io::ErrorKind::PermissionDenied => eprintln!("permission denied"),
                _ => eprintln!("unexpected error occured while reading the file"),
            }
            process::exit(1);
        });

    for line in matched {
        println!("{}", line);
    }
}
