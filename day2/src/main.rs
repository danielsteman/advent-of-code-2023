use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // let cases = read_lines("src/input.txt");
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let id = get_id(s);
    println!("id: {}", id);

    let sets = get_sets(s);
    println!("sets: {:?}", sets);
}

fn get_sets(s: &str) -> Vec<HashMap<&str, &str>> {
    let parts: Vec<&str> = s.split(": ").collect();
    let cubes = parts[1].split("; ");
    let mut sets: Vec<HashMap<&str, &str>> = vec![];

    for part in cubes {
        let mut map: HashMap<&str, &str> = HashMap::new();
        for cube in part.split(", ") {
            let cube_data: Vec<&str> = cube.split(" ").collect();
            let q = cube_data[0];
            let color = cube_data[1];
            map.insert(color, q);
        }
        sets.push(map);
    }

    sets
}

fn get_id(s: &str) -> &str {
    let parts: Vec<&str> = s.split(":").collect();
    let sub_parts: Vec<&str> = parts[0].split(" ").collect();
    sub_parts[1]
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
    sets: Vec<HashMap<&'a str, &'a str>>,
}

// impl Game {
//     fn new(s: &str) -> Game {}
// }
