// example:$ cargo run searchstring example-filename.txt
use std::env;
use std::fs;

fn main() {
    //collect arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //store args into variables
    let (query, filename) = parse_config(&args);
    
    //printing the variables to console for development
    println!("Searching for => {}", query);
    println!("In file => {}", filename);

    //read file, save text to var, and print to console
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong with reading the file");
    println!("With text:\n{}", contents);

}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
