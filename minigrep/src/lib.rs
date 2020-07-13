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


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("\n{}", contents);

    Ok(())
}


#[cfg(test)]
mod tests {
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
}
