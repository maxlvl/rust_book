use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args are {:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {:?}", query);
    println!("in file {:?}", filename);

    let contents: String = fs::read_to_string(filename).expect("Problem reading the file.");

    println!("With text content:\n\n{}", contents)
}
