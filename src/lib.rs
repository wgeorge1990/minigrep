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
    //TODO:

    // 1. Iterate through each line of the contents.
    let mut index = 0; //dev only
    for line in contents.lines() {
        index += 1; //dev only
        println!("{}. {}", index, line); //dev only
        // do something

    }

    // 2. Check whether the line contains our query string.
    // 3. If it does, add it to the list of values were returning.
    // 4. If it doesn't, do nothing.
    // 5. Return the list of results that match.
    vec![]
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
