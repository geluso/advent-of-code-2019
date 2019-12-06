use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    let contents = fs::read_to_string(filename)?;
    let codes: Vec<&str> = contents.trim().split(",").collect();

    let mut strip = vec![];
    for code in &codes {
        let nn = code.parse::<isize>().unwrap();
        strip.push(nn);
    }

    compute(&mut strip);

    Ok(())
}

fn compute(strip: &mut Vec<isize>) -> isize {
    let mut is_running = true;
    let mut index: usize = 0;
    while is_running {
        //println!("{:?}", strip);

        let instruction = strip[index];
        let op_code = instruction % 100;

        let p1_mode = instruction / 100 % 10;
        let p2_mode = instruction / 1000 % 10;
        let p3_mode = 0; // writable addresses are never in immediate mode.

        if op_code == 99 {
            is_running = false;
        } else if op_code == 1 || op_code == 2 {
            let n1 = strip[index + 1];
            let n2 = strip[index + 2];
            let n3 = strip[index + 3];

            //println!("op strip {} {} {} {}", instruction, n1, n2, n3);
            //println!("mode1: {}", p1_mode);
            //println!("mode2: {}", p2_mode);
            //println!("mode3: {}", p3_mode);

            let n1 = get_param(&strip, p1_mode, n1 as usize);
            let n2 = get_param(&strip, p2_mode, n2 as usize);
            let dest = n3 as usize;

            if op_code == 1 {
                //println!("{}+{} to {}", n1, n2, dest);
                add(strip, n1, n2, dest);
            } else if op_code == 2 {
                //println!("{}*{} to {}", n1, n2, dest);
                mult(strip, n1, n2, dest);
            }
            //println!("");

            index += 4;
        } else if op_code == 3 || op_code == 4 {
            let location = strip[index + 1] as usize;
            //println!("op strip {} {}", instruction, location);

            if op_code == 3 {
                input(strip, location);
            } else if op_code == 4 {
                let n1 = strip[index + 1];
                let n1 = get_param(&strip, p1_mode, n1 as usize);
                output(n1);
            }

            index += 2;
        } else if op_code == 5 || op_code == 6 {
            let n1 = strip[index + 1];
            let n2 = strip[index + 2];

            let n1 = get_param(&strip, p1_mode, n1 as usize);
            let n2 = get_param(&strip, p2_mode, n2 as usize);

            // if p1 != 0, index = p2
            if op_code == 5 {
               if n1 != 0 {
                    index = n2 as usize;
               } else {
                    index += 3;
               }
            // if p1 == 0, index = p2
            } else if op_code == 6 {
               if n1 == 0 {
                    index = n2 as usize;
               } else {
                    index += 3;
               }
            }
        } else if op_code == 7 || op_code == 8 {
            let n1 = strip[index + 1];
            let n2 = strip[index + 2];
            let location = strip[index + 3] as usize;

            let n1 = get_param(&strip, p1_mode, n1 as usize);
            let n2 = get_param(&strip, p2_mode, n2 as usize);

            // if p1 < p2 1=>@3 else 0=>@3
            if op_code == 7 {
                if n1 < n2 {
                    strip[location] = 1;
                } else {
                    strip[location] = 0;
                }
            // if p1 == p2 1=>@3 else 0=>@3
            } else if op_code == 8 {
                if n1 == n2 {
                    strip[location] = 1;
                } else {
                    strip[location] = 0;
                }
            }
            index += 4;
        }

        //println!("");
        //pause();
    }

    // what's the value at 0?
    return strip[0];
}

fn get_param(strip: &Vec<isize>, mode: isize, location: usize) -> isize { 
    // position mode
    if mode == 0 {
        //println!("@{}={}", location, strip[location]);
        return strip[location];
    }

    // immediate mode
    //println!("lit{}", location);
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
    // println!("input:");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let input = line.trim().parse::<isize>().unwrap();
    strip[dest] = input;
}

fn output(value: isize) {
    println!("{}", value);
}

fn pause() {
    println!("continue:");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
