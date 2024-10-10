use colored::Colorize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Config {
    pub query: String,
    pub file_path: std::path::PathBuf,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: String, file_path: std::path::PathBuf, ignore_case: bool) -> Config {
        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn run<F>(&self, mut line_handler: F) -> io::Result<()>
    where
        F: FnMut(&str) -> io::Result<()>,
    {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            line_handler(&line)?;
        }

        Ok(())
    }
}

pub fn colorize(query: &str, line: &str, ignore_case: bool) -> String {
    let mut result = String::new();
    let mut last_match_end: usize = 0;

    let (search_line, search_query) = if ignore_case {
        (line.to_lowercase(), query.to_lowercase())
    } else {
        (line.to_string(), query.to_string())
    };

    for (start, matched) in search_line.match_indices(search_query.as_str()) {
        result.push_str(&line[last_match_end..start]);
        result.push_str(&line[start..start + matched.len()].bold().red().to_string());
        last_match_end = start + matched.len();
    }

    result.push_str(&line[last_match_end..]);
    result
}
