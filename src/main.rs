// example:$ cargo run searchstring example-filename.txt
use std::env;
use std::fs;

fn main() {
    //collect arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //store args into variables
    let config = parse_config(&args);
    
    //printing the variables to console for development
    println!("Searching for => {}", config.query);
    println!("In file => {}", config.filename);

    //read file, save text to var, and print to console
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong with reading the file");
    println!("With text:\n{}", contents);

}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}
