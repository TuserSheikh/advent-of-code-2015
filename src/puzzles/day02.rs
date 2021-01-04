use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part_one() -> i32 {
    let mut paper = 0;
    let file_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input/02.txt";
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(dimensions) = line {
                let mut d = dimensions
                    .split("x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                d.sort();
                let (h, w, l) = (d[0], d[1], d[2]);
                paper += (h * w) + (2 * l * w) + (2 * w * h) + (2 * h * l);
            }
        }
    }
    paper
}

pub fn part_two() -> i32 {
    let mut paper = 0;
    let file_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input/02.txt";
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(dimensions) = line {
                let mut d = dimensions
                    .split("x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                d.sort();
                let (h, w, l) = (d[0], d[1], d[2]);
                paper += h + h + w + w + (h * w * l);
            }
        }
    }
    paper
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
