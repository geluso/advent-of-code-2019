use std::env;
use std::fs;
use std::io::{self, BufRead};

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
    // println!("codes: {:?}", codes);

    let mut strip = vec![];
    for code in &codes {
        let nn = code.parse::<isize>().unwrap();
        strip.push(nn);
    }

    compute(&mut strip);

    // old searching code
    //
    // let seeking = 19690720;
    // for noun in 0..=99 {
    //     for verb in 0..=99 {
    //         strip[1] = noun;
    //         strip[2] = verb;

    //         let mut fresh_strip = strip.clone();
    //         let result = compute(&mut fresh_strip);
    //         if result == seeking {
    //             println!("{} {}", noun, verb);                       
    //         }
    //     }
    // }

    Ok(())
}

fn compute(strip: &mut Vec<isize>) -> isize {
    let mut is_running = true;
    let mut index: usize = 0;
    while is_running {
        println!("{:?}", strip);

        let instruction = strip[index];
        let op_code = instruction % 100;

        let p1_mode = instruction / 100 % 10;
        let p2_mode = instruction / 1000 % 10;
        let p3_mode = 0; // writable addresses are never in immediate mode.

        println!("op: {}", op_code);

        if op_code == 99 {
            is_running = false;
        } else if op_code == 1 || op_code == 2 {
            let n1 = strip[index + 1];
            let n2 = strip[index + 2];
            let n3 = strip[index + 3];

            println!("op strip {} {} {} {}", op_code, n1, n2, n3);
            println!("mode1: {}", p1_mode);
            println!("mode2: {}", p2_mode);
            println!("mode3: {}", p3_mode);

            let n1 = get_param(&strip, p1_mode, n1 as usize);
            let n2 = get_param(&strip, p2_mode, n2 as usize);
            let dest = n3 as usize;

            if op_code == 1 {
                println!("{}+{} to {}", n1, n2, dest);
                add(strip, n1, n2, dest);
            } else if op_code == 2 {
                println!("{}*{} to {}", n1, n2, dest);
                mult(strip, n1, n2, dest);
            }
            println!("");

            index += 4;
        } else if op_code == 3 {
            let location = strip[index + 1] as usize;

            if op_code == 3 {
                input(strip, location);
            } else if op_code == 4 {
                let value = strip[location];
                output(value);
            }

            index += 2;
        }
        println!("");
    }

    // what's the value at 0?
    return strip[0];
}

fn get_param(strip: &Vec<isize>, mode: isize, location: usize) -> isize { 
    // position mode
    if mode == 0 {
        return strip[location];
    }

    // immediate mode
    return location as isize;
}

fn add(strip: &mut Vec<isize>, n1: isize, n2: isize, dest: usize) {
    let sum = n1 + n2;
    strip[dest] = sum;
}

fn mult(strip: &mut Vec<isize>, n1: isize, n2: isize, dest: usize) {
    let product = n1 * n2;
    strip[dest] = product;
}

fn input(strip: &mut Vec<isize>, dest: usize) {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let input = line.trim().parse::<isize>().unwrap();
    strip[dest] = input;
}

fn output(value: isize) {
    println!("{}", value);
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
