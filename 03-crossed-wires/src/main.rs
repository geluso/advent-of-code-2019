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

    let points1 = path_points(&path1);
    let points2 = path_points(&path2);

    let mut is_first = true;
    let mut min_distance = 0;

    for intersect in points1.intersection(&points2) {
        let coords: Vec<&str> = intersect.split(",").collect();
        let mut xx = coords[0].parse::<i32>().unwrap();
        let mut yy = coords[1].parse::<i32>().unwrap();

        if xx < 0 {
            xx = -xx;
        }

        if yy < 0 {
            yy = -yy;
        }

        let distance = xx + yy;
        if is_first || distance < min_distance {
            is_first = false;
            min_distance = distance;
        }
    }

    println!("{}", min_distance);

    Ok(())
}

fn path_points(path: &Vec<&str> ) -> HashSet<String> {
    let mut points: HashSet<String> = HashSet::new();

    let mut xx: isize = 0;
    let mut yy: isize = 0;
    for direction in path {
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

        let coord = format!("{},{}", xx, yy);
        path.insert(coord);
    }


    (xx, yy)
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
