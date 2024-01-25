use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub struct Config<'a> {
    query: &'a str,
    ignore_case: bool,
    read_mode: ReadMode<'a>,
}

pub enum ReadMode<'a> {
    File(&'a str),
    Stream(),
}

impl Config<'_> {
    pub fn new<'a>(query: &'a str, file_name: &'a str, ignore_case: bool) -> Config<'a> {
        Config::from_file(query, file_name, ignore_case)
    }

    pub fn from_file<'a>(query: &'a str, file_name: &'a str, ignore_case: bool) -> Config<'a> {
        Config {
            query,
            read_mode: ReadMode::File(file_name),
            ignore_case,
        }
    }

    pub fn from_stream<'a>(query: &'a str, ignore_case: bool) -> Config<'a> {
        Config {
            query,
            read_mode: ReadMode::Stream(),
            ignore_case,
        }
    }
}

pub fn run(cfg: &Config) -> Result<(), Box<dyn Error>> {
    let run_fn = match cfg.read_mode {
        ReadMode::File(file_name) => run_file(cfg.query, file_name, cfg.ignore_case),
        ReadMode::Stream() => run_stream(cfg.query, cfg.ignore_case),
    }?;

    for i in run_fn {
        println!("{i}");
    }

    Ok(())
}

pub fn run_stream<'a>(
    query: &'a str,
    ignore_lowercase: bool,
) -> Result<Box<dyn Iterator<Item = String> + 'a>, Box<dyn Error>> {
    let stdin = io::stdin();
    let base = stdin.lock().lines().filter_map(|line| line.ok());

    Ok(search(query, base, ignore_lowercase))
}

pub fn run_file<'a>(
    query: &'a str,
    file_name: &'a str,
    ignore_lowercase: bool,
) -> Result<Box<dyn Iterator<Item = String> + 'a>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let base = reader.lines().filter_map(|line| line.ok());

    Ok(search(query, base, ignore_lowercase))
}

pub fn search<'a>(
    query: &'a str,
    contents: impl Iterator<Item = String> + 'a,
    ignore_lowercase: bool,
) -> Box<dyn Iterator<Item = String> + 'a> {
    if query.is_empty() {
        return Box::new(std::iter::empty());
    }

    let new_query = if ignore_lowercase {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    Box::new(contents.filter(move |line| {
        if ignore_lowercase {
            line.to_lowercase().contains(&new_query)
        } else {
            line.contains(&new_query)
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Ducttape is cool!";

    fn prepare_test_contents() -> impl Iterator<Item = String> {
        CONTENTS.split("\n").map(|x| x.to_string())
    }

    #[test]
    fn case_sensitive_search_finds_exact_match() {
        let query = "Ducttape is cool!";
        let iter = prepare_test_contents();
        let expected = vec![query];
        let result = search(query, iter, false).collect::<Vec<String>>();
        assert_eq!(
            result, expected,
            "Case sensitive search should find an exact match for '{}'",
            query
        );
    }

    #[test]
    fn case_sensitive_search_no_results() {
        let query = "nonexistent";
        let iter = prepare_test_contents();
        let expected: Vec<String> = vec![];
        let result = search(query, iter, false).collect::<Vec<String>>();
        assert_eq!(
            result, expected,
            "Case sensitive search should return no results for '{}'",
            query
        );
    }

    #[test]
    fn case_insensitive_search_no_results() {
        let query = "nonexistent";
        let iter = prepare_test_contents();
        let expected: Vec<String> = vec![];
        let result = search(query, iter, true).collect::<Vec<String>>();
        assert_eq!(
            result, expected,
            "Case insensitive search should return no results for '{}'",
            query
        );
    }

    #[test]
    fn search_with_empty_query() {
        let query = "";
        let iter = prepare_test_contents();
        let expected: Vec<String> = vec![];
        let result = search(query, iter, true).collect::<Vec<String>>();
        assert_eq!(
            result, expected,
            "Search with an empty query should return no results"
        );
    }

    #[test]
    fn search_in_empty_contents() {
        let query = "Rust";
        let iter = "".split("\n").map(String::from);
        let expected: Vec<String> = vec![];
        let result = search(query, iter, false).collect::<Vec<String>>();
        assert_eq!(
            result, expected,
            "Search in empty contents should return no results for '{}'",
            query
        );
    }
}
