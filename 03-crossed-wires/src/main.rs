use std::env;
use std::fs;
use std::io;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct Coord {
    xx: isize,
    yy: isize,
    delay: usize
}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.xx.hash(state);
        self.yy.hash(state);
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.xx == other.xx && self.yy == other.yy
    }
}

impl Eq for Coord {}

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

    for coord in points1.keys() {
        if !points2.contains_key(&coord) {
            continue;
        }

        let coord2 = points2.get(coord).unwrap();
        let distance = coord.delay + coord2.delay;

        if is_first || distance < min_distance {
            is_first = false;
            min_distance = distance;
        }
    }

    println!("{}", min_distance);

    Ok(())
}

fn path_points(path: &Vec<&str> ) -> HashMap<Coord, Coord> {
    let mut points: HashMap<Coord, Coord> = HashMap::new();

    let mut xx: isize = 0;
    let mut yy: isize = 0;
    let mut steps: usize = 0;
    for direction in path {
        let (aa, bb, cc) = point_to_point(&mut points, direction, xx, yy, steps);
        xx = aa;
        yy = bb;
        steps = cc;
    }

    return points;
}

fn point_to_point(
        path: &mut HashMap<Coord, Coord>, direction: &str,
        mut xx: isize, mut yy: isize, mut steps: usize
    ) -> (isize, isize, usize) {
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
        steps += 1;
        xx += dx;
        yy += dy;

        // let coord = format!("{},{}", xx, yy);
        let coord = Coord {
            xx: xx,
            yy: yy,
            delay: steps,
        };

        if !path.contains_key(&coord) {
            let key = coord.clone();
            let val = coord.clone();
            path.insert(key, val);
        }
    }

    (xx, yy, steps)
}

fn parse_config(args: &[String]) -> (&str) {
    let filename = &args[1];
    (filename)
}
