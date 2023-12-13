use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // let cases = read_lines("src/input.txt");
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let parts: Vec<&str> = s.split(":").collect();

    if let Some(id) = parts.get(0) {
        println!("ID: {}", id.trim());
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

struct Game<'a> {
    id: i32,
    sets: Vec<HashMap<i32, &'a str>>,
}

// impl Game {
//     fn new(s: &str) -> Game {}
// }
