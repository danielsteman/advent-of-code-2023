use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let rules = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let cases = read_lines("src/input.txt");
    let mut sum = 0;

    for case in &cases {
        let game = Game::new(case);
        let res = game.is_valid(&rules);

        println!("res: {}", res);
        println!("sum: {}", sum);
        println!("game id: {}", game.id);

        sum += res;
    }

    println!("Sum: {}", sum);
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

                let counter_value = counter.get(key).unwrap();
                let rule_value = rules.get(key).unwrap();

                if *counter_value > *rule_value {
                    println!(
                        "{:?}: {:?} is greater than {:?} which is {:?}",
                        key,
                        counter_value,
                        rule_value,
                        counter_value > rule_value
                    );
                    println!("Violated rules, returning zero. Counter: {:?}", counter);
                    println!("");
                    return &0;
                }
            }
            // println!("{:?}", set);
        }

        // println!("counter {:?}", counter);

        &self.id
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
