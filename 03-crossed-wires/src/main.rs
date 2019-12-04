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

    println!("path1 {:?}", &path1);
    println!("path2 {:?}", &path2);

    let points1 = path_points(&path1);
    let points2 = path_points(&path2);

    Ok(())
}

fn path_points(path: &Vec<&str> ) -> HashSet<String> {
    let mut points: HashSet<String> = HashSet::new();
    points.insert("(0,0)".to_string());

    let mut xx: isize = 0;
    let mut yy: isize = 0;
    for direction in path {
        println!("{}", direction);
        let (aa, bb) = point_to_point(&mut points, direction, xx, yy);
        xx = aa;
        yy = bb;
    }

    return points;
}

fn point_to_point(path: &mut HashSet<String>, direction: &str, mut xx: isize, mut yy: isize) -> (isize, isize) {
    let mut chars = direction.chars();
    let letter = chars.next().unwrap();
    let nn = chars.as_str().parse::<u32>().unwrap();

    let mut dx = 0;
    let mut dy = 0;

    if letter == 'R' {
        dx = 1;
    } else if letter == 'L' {
        dx = -1;
    } else if letter == 'U' {
        dy = 1;
    } else if letter == 'D' {
        dy = -1;
    }

    for _ in 1..=nn {
        xx += dx;
        yy += dy;

        let coord = format!("({},{})", xx, yy);
        println!("{}", coord);
        path.insert(coord);
    }


    (xx, yy)
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
