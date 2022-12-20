use std::fs;
use std::error::Error;
//accessing enviornement variables
use std::env;

//read file, save text to var, and print to console
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    
    // implementing case_sensitive attribut on config
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    // using search function in run function.
    for line  in results {
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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
     Ok(Config { query, filename, case_sensitive})
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec< &'a str> {
    let mut results = Vec::new();
    // 1. Iterate through each line of the contents.
    for line in contents.lines() {
        // 2. Check whether the line contains our query string.
        if line.contains(query) { //contains() sig is for &str
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); //query is converted to String by to_lowercase.
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { //becasue of conversion we need to reference as str slice now.
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
