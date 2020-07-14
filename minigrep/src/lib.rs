use std::{env, fs};
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive })
    }    
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_new_config() {
        let query = String::from("regex");
        let filename = String::from("file.txt");

        let config = Config::new(&[String::from("path/to/file"), query.clone(), filename.clone()])
            .unwrap();

        assert_eq!(query, config.query);
        assert_eq!(filename, config.filename);
    }

    #[test]
    fn test_run() {
        let config = Config::new(&[String::from("path/to/file"), String::from("file.txt"), String::from("regex")])
            .unwrap();

        if let Ok(_) = run(config) {
            panic!("Expected error. Should have failed to read file.");
        }
    }

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = indoc!("\
            Rust
            safe, fast, productive.
            Pick three.
            Duct tape.
        ");

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_insensitive() {
        let query = "rUsT";
        let contents = indoc!("\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.
        ");

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
