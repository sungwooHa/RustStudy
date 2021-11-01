
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    let query = &args[1];
    let filename = &args[2];

    println!("searching for {}", query);
    println!("in file {}", filename);
}
