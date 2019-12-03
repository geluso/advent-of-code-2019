use std::env;
use std::fs;

use std::fs::File;

use std::io;
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Searching for: {}", query);
    println!("In file: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);


    println!("");
    println!("Lines");
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
