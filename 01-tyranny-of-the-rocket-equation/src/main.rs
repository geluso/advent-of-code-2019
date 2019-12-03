use std::env;
use std::fs::File;

use std::io;
use std::i32;
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    // total up all the module weights
    let mut total_fuel = 0;
    for line in buffered.lines() {
        let weight = line.unwrap().parse::<i32>().unwrap();
        let fuel = fuel_cost(weight);
        total_fuel += fuel;

        let mut additional_fuel = fuel_cost(fuel);
        while additional_fuel > 0 {
            total_fuel += additional_fuel;
            additional_fuel = fuel_cost(additional_fuel);
        }
    }


    println!("{}", total_fuel);

    Ok(())
}

fn fuel_cost(weight: i32) -> i32 {
    let div = weight / 3;
    let two = div - 2;
    if two > 0 {
        return two;
    } else {
        return 0;
    }
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
