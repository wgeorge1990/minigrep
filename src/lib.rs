use std::fs;
use std::error::Error;

//read file, save text to var, and print to console
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
     Ok(Config { query, filename})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec< &'a str> {
    let mut results = Vec::new();
    // 1. Iterate through each line of the contents.
    for line in contents.lines() {
        // 2. Check whether the line contains our query string.
        if line.contains(query) {
                // 3. If it does, add it to the list of values were returning.
            results.push(line);
            println!("line: {}, contains query: {}", line, query); //dev only
            // do something with line
        }
        // 4. If it doesn't, do nothing.
    }
    // 5. Return the list of results that match.
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
