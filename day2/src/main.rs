use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // let cases = read_lines("src/input.txt");
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let id = get_id(s);
    println!("id: {}", id);

    let sets = get_sets(s);
    println!("sets: {:?}", sets);

    let game = Game::new(s);
    println!("game: {:?}", game);

    let rules = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    println!("rules: {:?}", rules);

    game.is_valid(&rules);
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

fn get_id(s: &str) -> i32 {
    let parts: Vec<&str> = s.split(":").collect();
    let sub_parts: Vec<&str> = parts[0].split(" ").collect();
    let parsed: i32 = sub_parts[1].parse::<i32>().expect("Couldn't parse");
    parsed
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Debug)]
struct Game<'a> {
    id: i32,
    sets: Vec<HashMap<&'a str, &'a str>>,
}

impl Game<'_> {
    fn new(s: &str) -> Game {
        let id = get_id(s);
        let sets = get_sets(s);

        Game { id, sets }
    }

    fn is_valid(&self, rules: &HashMap<&str, i32>) -> &i32 {
        let mut counter = rules.clone();
        for (_, value) in counter.iter_mut() {
            *value = 0;
        }

        for set in &self.sets {
            for (key, value) in set.iter() {
                *counter.entry(key).or_insert(0) += value
                    .parse::<i32>()
                    .expect("Couldn't parse number of cubes");
            }
            println!("{:?}", set);
        }

        println!("counter {:?}", counter);

        &self.id
    }
}
