use std::env;
use std::fs;
use std::io;

use std::collections::HashSet;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    let contents = fs::read_to_string(filename)?;
    let paths: Vec<&str> = contents.trim().split("\n").collect();

    let path1: Vec<&str> = paths[0].split(",").collect();
    let path2: Vec<&str> = paths[1].split(",").collect();

    println!("path1 {:?}", path1);
    println!("path2 {:?}", path2);

    let mut path1_points = HashSet::new();
    let mut path2_points = HashSet::new();

    path1_points.insert("(0,0");
    path2_points.insert("(0,0");

    let mut xx: isize = 0;
    let mut yy: isize = 0;
    for direction in path1 {
        // why can't I assign directly back to xx and yy?
        //let (aa, bb) = point_to_point(xx, yy, direction);
        //xx = aa;
        //yy = bb;
        (xx, yy) = point_to_point(xx, yy, direction);
        println!("{} {}", xx, yy);
    }

    Ok(())
}

fn point_to_point(xx: isize, yy: isize, direction: &str) -> (isize, isize) {
    let letter = direction.chars().next().unwrap();
    if letter == 'R' {
        return (xx + 1, yy);
    } else if letter == 'L' {
        return (xx - 1, yy);
    } else if letter == 'U' {
        return (xx, yy + 1);
    } else if letter == 'D' {
        return (xx, yy - 1);
    }

    (xx, yy)
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
