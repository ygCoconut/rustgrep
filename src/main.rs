// We will create a program that works like grep.
// It will allow you to find words in textfiles

// see below how grep works:
// >rustgrep(main)$ grep print src/main.rs
//println!("{:?}",args);                                  println!("Searching for {} in file {}", query, filename);


use std::env; // allows to collect cmd line args
use std::process;
use rustgrep::{Config, run}; //lib.rs

fn main() {
    let args: Vec<String> = env::args().collect();

    // get args:
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in file {}", config.query, config.filename);

    // run(config); with error handling
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    } // if let instead of unwrap_or_else()

}
