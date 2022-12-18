// example:$ cargo run searchstring example-filename.txt
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //collect arguments from command line
    let args: Vec<String> = env::args().collect();

    //create Config struct with parsed arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}