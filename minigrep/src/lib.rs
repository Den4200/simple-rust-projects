use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
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


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("\nContents:\n{}", contents);

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
    fn test_search() {
        let query = "duct";
        let contents = indoc!("\
            Rust
            safe, fast, productive.
            Pick three.
        ");

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
