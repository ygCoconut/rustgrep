use std::fs;   // read files
use std::error;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // remember 'static lifetimes live for the entire duration of the program, which is fine here.
        if args.len() < 3 {
            return Err("Not enough arguments\ntry: rustgrep <query> <filename>")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // clone allows Config to take ownership
        // N.B.: cloning a full copy of the data gives up
            // a little bit of performance, but makes the code
            // way more readable.
        Ok(Config {query, filename})
    }
}



// Result<(), Box<dyn error::Error>>
// is a good pick for the run() function !
pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    // Box<dyn Error> means the function will return a
    // type that implements the Error trait, but we don’t
    // have to specify what particular type the return
    // value will be.

    // read_to_string returns a Result
    let contents = fs::read_to_string(config.filename)?;
    let results = search(&config.query, &contents);
    for line in results {
        println!("{}", line);
    }
    // println!("With text:\n{}", contents);

    Ok(())
}

// notice we need an explicit lifetime here ('a)
// In other words, we tell Rust that the data returned by the
// search function will live as long as the data passed into
// the search function in the contents argument.
// Also contents is the component that should be connected to the
// line of output.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            found.push(line);
        }
    }
    found
}



// Approach used here TDD = Test-Driven Development
// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";

        // Assert the line we expect to contain the term is the right one.
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        // We need to create the search function, otherwhise the test won't
        // compile and we can't even watch it fail..
    }
}

