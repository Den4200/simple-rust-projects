use std::{env, fs};
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}


impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query argument")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename argument")
        };

        let case_sensitive;
        match args.next() {
            Some(arg) => {
                case_sensitive = match arg.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err("Incorrect argument for case sensitive parameter.")
                }
            }
            None => {
                case_sensitive = match env::var("CASE_INSENSITIVE") {
                    Ok(value) => if value == "1" { false } else { true },
                    Err(_) => false
                };
            }
        }    

        Ok(Config { query, filename, case_sensitive })
    }    
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
