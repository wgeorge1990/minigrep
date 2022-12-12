// example:$ cargo run searchstring example-filename.txt
use std::env;

fn main() {
    //collect arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //store args into variables
    let query = &args[1];
    let filename = &args[2];
    
    //printing the variables to console for development
    print!("Searching for {}", query);
    println!("In file {}", filename);
}
