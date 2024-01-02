use std::cmp;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let rules = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let cases = read_lines("src/input.txt");
    let mut sum = 0;
    let mut power_sum = 0;

    for case in &cases {
        let game = Game::new(case);
        let res = game.get_points(&rules);
        let power_points = game.get_power_points();

        sum += res;
        power_sum += power_points
    }

    println!("Sum: {}", sum);
    println!("Power sum: {}", power_sum);
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
        let id = Game::get_id(s);
        let sets = Game::get_sets(s);

        Game { id, sets }
    }

    fn get_points(&self, rules: &HashMap<&str, i32>) -> &i32 {
        for set in &self.sets {
            for (key, value) in set.iter() {
                let parsed_value = value
                    .parse::<i32>()
                    .expect("Couldn't parse number of cubes");

                let rule_value = rules.get(key).unwrap();

                if parsed_value > *rule_value {
                    println!("Violated rules, returning zero");
                    return &0;
                }
            }
        }

        &self.id
    }

    fn get_power_points(&self) -> i32 {
        let mut counter: HashMap<&str, i32> = HashMap::new();

        for set in &self.sets {
            for (key, value) in set.iter() {
                let counter_value = *counter.entry(key).or_insert(0);
                let parsed_value = value
                    .parse::<i32>()
                    .expect("Couldn't parse number of cubes");
                counter.insert(key, cmp::max(counter_value, parsed_value));
            }
        }

        let mut product = 1;
        for (_, &value) in counter.iter() {
            product *= value;
        }

        product
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
}
