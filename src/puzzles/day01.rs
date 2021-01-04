use std::env;
use std::fs;

pub fn part_one() -> i32 {
    let file_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input/01.txt";
    let directions = fs::read_to_string(file_path).unwrap();

    let mut floor = 0;
    for i in directions.chars() {
        match i {
            '(' => floor += 1,
            _ => floor -= 1,
        }
    }

    floor
}

pub fn part_two() -> i32 {
    let file_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input/01.txt";
    let directions = fs::read_to_string(file_path).unwrap();

    let mut floor = 0;
    for (i, c) in directions.chars().enumerate() {
        match c {
            '(' => floor += 1,
            _ => floor -= 1,
        }

        if floor == -1 {
            return i as i32 + 1;
        }
    }

    0
}
