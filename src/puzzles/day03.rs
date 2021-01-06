use std::collections::HashSet;
use std::env;
use std::fs;

struct Position {
    x: i32,
    y: i32,
}
struct Santa {
    x: i32,
    y: i32,
}
struct Robo {
    x: i32,
    y: i32,
}

pub fn part_one() -> usize {
    let file_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input/03.txt";
    let moves = fs::read_to_string(file_path).unwrap();

    let mut houses = HashSet::new();
    let mut pos = Position { x: 0, y: 0 };

    for m in moves.chars() {
        match m {
            '^' => pos.y += 1,
            '>' => pos.x += 1,
            'v' => pos.y -= 1,
            _ => pos.x -= 1,
        }
        houses.insert((pos.x, pos.y));
    }

    houses.len()
}

pub fn part_two() -> usize {
    let file_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input/03.txt";
    let moves = fs::read_to_string(file_path).unwrap();

    let mut houses = HashSet::new();
    let mut santa = Santa { x: 0, y: 0 };
    let mut robo = Robo { x: 0, y: 0 };

    for (i, m) in moves.chars().enumerate() {
        if i % 2 == 0 {
            match m {
                '^' => santa.y += 1,
                '>' => santa.x += 1,
                'v' => santa.y -= 1,
                _ => santa.x -= 1,
            }
            houses.insert((santa.x, santa.y));
        } else {
            match m {
                '^' => robo.y += 1,
                '>' => robo.x += 1,
                'v' => robo.y -= 1,
                _ => robo.x -= 1,
            }
            houses.insert((robo.x, robo.y));
        }
    }

    houses.len()
}
