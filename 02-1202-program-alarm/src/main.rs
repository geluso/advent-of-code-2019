use std::env;
use std::fs;
use std::io;

// 99 means that the program is finished and should immediately halt. Encountering an unknown
// opcode means something went wrong.

// Opcode 1 adds together numbers read from two positions and stores the result in a third
// position. The three integers immediately after the opcode tell you these three positions - the
// first two indicate the positions from which you should read the input values, and the third
// indicates the position at which the output should be stored.
// 
// For example, if your Intcode computer encounters 1,10,20,30, it should read the values at
// positions 10 and 20, add those values, and then overwrite the value at position 30 with their
// sum.
// 
// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding
// them. Again, the three integers after the opcode indicate where the inputs and outputs are, not
// their values.
// 
// Once you're done processing an opcode, move to the next one by stepping forward 4 positions.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    let contents = fs::read_to_string(filename)?;
    let codes: Vec<&str> = contents.trim().split(",").collect();

    // print a vector with {:?}
    // pretty print a vector with {:#?}
    println!("codes: {:?}", codes);

    let mut strip = vec![];
    for code in &codes {
        let nn = code.parse::<usize>().unwrap();
        strip.push(nn);
    }
    println!("codes: {:?}", strip);

    // replace value 1 with 12
    // replace value 2 with 02
    strip[1] = 12;
    strip[2] = 2;

    let mut is_running = true;
    let mut index = 0;
    while is_running {
        println!("{}", strip[index]);

        let op_code = strip[index];
        if op_code == 99 {
            println!("quit");
            is_running = false;
        } else {
            let i1 = strip[index + 1];
            let i2 = strip[index + 2];
            let dest = strip[index + 3];
            let mut val = strip[dest];
            println!("was: {} {} i[{}]={}", i1, i2, dest, val);

            if op_code == 1 {
                println!("add");
                add(&mut strip, index);
            } else if op_code == 2 {
                println!("mult");
                mult(&mut strip, index);
            }

            val = strip[dest];
            println!("now: {} {} i[{}]={}", i1, i2, dest, val);

            index += 4
        }

        println!("codes: {:?}", strip);
    }

    // what's the value at 0?
    println!("{}", strip[0]);

    Ok(())
}

fn add(strip: &mut Vec<usize>, index: usize) {
    let n1: usize = strip[index + 1];
    let n2: usize = strip[index + 2];
    let dest: usize = strip[index + 3];

    let sum = strip[n1] + strip[n2];
    strip[dest] = sum;
}

fn mult(strip: &mut Vec<usize>, index: usize) {
    let n1: usize = strip[index + 1];
    let n2: usize = strip[index + 2];
    let dest: usize = strip[index + 3];

    let product = strip[n1] * strip[n2];
    strip[dest] = product;
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
