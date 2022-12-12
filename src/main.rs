// example:$ cargo run searchstring example-filename.txt
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];
    print!("Searching for {}", query);
    println!("In file {}", filename);
}
