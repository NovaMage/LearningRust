use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments\nUsage: minigrep [QUERY] [FILENAME]\ne.g. minigrep the somefile.txt\n");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = &query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture {
        query: &'static str,
        contents: &'static str,
    }

    impl Fixture {
        fn new() -> Fixture {
            Fixture {
                query: "duct",
                contents: "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.",
            }
        }
    }

    #[test]
    fn case_sensitive() {
        let fixture = Fixture::new();
        assert_eq!(
            vec!["safe, fast, productive."],
            search(fixture.query, fixture.contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let fixture = Fixture::new();
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(fixture.query, fixture.contents)
        );
    }
}
