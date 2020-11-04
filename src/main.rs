// We will create a program that works like grep.
// It will allow you to find words in textfiles

// see below how grep works:
// >rustgrep(main)$ grep print src/main.rs
//println!("{:?}",args);                                  println!("Searching for {} in file {}", query, filename);

use rustgrep::{run, Config};
use std::env; // allows to collect cmd line args
use std::process; //lib.rs

fn main() {
    let args: Vec<String> = env::args().collect();

    // get args:
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln! prints to stderr instead of stdout !
        // cargo run > output.txt
        // will write error to the terminal,
        // while stdoutput will be written to output.txt
        // output.txt will remain empty in this case.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {} in file {}", config.query, config.filename);

    // run(config); with error handling
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } // if let instead of unwrap_or_else()
}
